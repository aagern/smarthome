# Network Devices Implementation Plan (DZ3)

> **For agentic workers:** Use `superpowers:executing-plans` or `superpowers:subagent-driven-development` to execute this plan task-by-task.

**Goal:** Add real TCP/UDP network communication to `SmartSocket` and `SmartThermometer`, plus simulator binaries and a demo example.

**Architecture decisions (confirmed by teacher):**
- **Binary protocol** over TCP — raw bytes (u8 command, binary responses), not text lines.
- **Full Tokio** — all network I/O is async, including device client code and config-file reading.
- **Workspace layout** — simulator binaries live in separate crates (`socket-sim/`, `thermo-sim/`) alongside the existing library crate.
- **Error handling** — every network function returns `Result`. Errors bubble up to the caller. At the application level (demo/main), the caller decides whether to panic or mark the device faulty and continue.

---

## The Big Picture

Right now `SmartSocket` and `SmartThermometer` are dumb structs that return random numbers. DZ3 turns them into real async network clients:

```
SmartSocket  ←── TCP (binary) ──→  socket-sim   (separate crate, runs as a fake device)
SmartThermometer ←── UDP (binary) ── thermo-sim  (separate crate, sends temperature packets)
```

Old code is untouched: `mock.rs` stays as-is, `lib.rs` just gets one new line (`pub mod net;`).

### Binary protocol — why and what

Text protocols (like HTTP) are easy to read but waste bytes parsing strings. Real embedded devices speak binary: they send a single byte meaning "give me power" and get back 4 bytes containing the float value. The teacher calls this "stream of bits like in production systems."

**TCP socket protocol (u8 request → binary response):**
| Request byte | Meaning | Response |
|---|---|---|
| `0x01` | Is socket on? | 1 byte: `0x00` = off, `0x01` = on |
| `0x02` | Get power | 4 bytes f32 little-endian |
| `0x03` | Turn on | 1 byte: `0x00` = OK |
| `0x04` | Turn off | 1 byte: `0x00` = OK |

**UDP thermometer protocol:** each datagram is exactly 4 bytes — a `f32` in little-endian byte order.

---

## Step 0 — Reorganise into a Cargo workspace

**Files:** root `Cargo.toml`, new `socket-sim/Cargo.toml`, new `thermo-sim/Cargo.toml`

A workspace lets you have multiple crates in one repo that share a build cache and can depend on each other. Right now the repo is a single package. We turn the root into a workspace so the simulator binaries live as independent crates.

**Edit the root `Cargo.toml`** — add a `[workspace]` block at the very top (before `[package]`):

```toml
[workspace]
members = [
    ".",          # the existing library/binary package stays here
    "socket-sim",
    "thermo-sim",
]
resolver = "2"
```

**Create `socket-sim/Cargo.toml`:**

```toml
[package]
name = "socket-sim"
version = "0.1.0"
edition = "2024"

[dependencies]
smarthouse = { path = ".." }   # re-use protocol constants from the lib
tokio = { version = "1", features = ["net", "rt-multi-thread", "macros", "time", "io-util"] }
rand = "0.9"
tracing = "0.1"
tracing-subscriber = "0.3"
```

**Create `thermo-sim/Cargo.toml`:**

```toml
[package]
name = "thermo-sim"
version = "0.1.0"
edition = "2024"

[dependencies]
smarthouse = { path = ".." }   # re-use protocol constants
tokio = { version = "1", features = ["net", "rt-multi-thread", "macros", "time", "io-util", "fs"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
rand = "0.9"
tracing = "0.1"
tracing-subscriber = "0.3"
```

Think of workspace members like sub-folders in a monorepo: they share `Cargo.lock` (so everyone pins the same dependency versions) but each compiles separately.

---

## Step 1 — Add dependencies to the root `Cargo.toml`

**File:** `Cargo.toml` (`[dependencies]` section of the existing library package)

```toml
[dependencies]
# ... (keep everything already there)
tokio = { version = "1", features = ["net", "rt-multi-thread", "macros", "time", "io-util"] }
```

No `serde`/`toml` here — those belong only in `thermo-sim` (config reading lives there).

---

## Step 2 — Create `src/net/` module skeleton

**Files:** `src/net/mod.rs`, `src/net/protocol.rs`, one line added to `src/lib.rs`

Tell the library that a `net` module exists. In `src/lib.rs`, add after the existing `pub mod legacy;` / `pub mod new;` lines:

```rust
pub mod net;
```

Create `src/net/mod.rs`:

```rust
pub mod protocol;
pub mod socket;
pub mod thermometer;

pub use socket::SmartSocket;
pub use thermometer::SmartThermometer;
```

Create `src/net/protocol.rs` — binary command and response constants shared between the library and both simulator crates:

```rust
// Commands the client sends (1 byte each):
pub const CMD_IS_ON: u8   = 0x01;
pub const CMD_POWER: u8   = 0x02;
pub const CMD_TURN_ON: u8  = 0x03;
pub const CMD_TURN_OFF: u8 = 0x04;

// Response values:
pub const RESP_OFF: u8 = 0x00;
pub const RESP_ON:  u8 = 0x01;
pub const RESP_OK:  u8 = 0x00;
```

Because both the library and the simulators import these from `smarthouse::net::protocol`, you can never accidentally have a mismatch between what the client sends and what the server expects.

---

## Step 3 — Async TCP `SmartSocket` client in `src/net/socket.rs`

**File:** `src/net/socket.rs`

The socket connects to a TCP server, sends a 1-byte command, reads the response bytes, and closes the connection. All I/O is async (tokio). Each call opens its own connection — simple and robust.

```rust
use crate::net::protocol::*;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

struct MockState {
    is_on: bool,
    power: f32,
}

enum SocketMode {
    Tcp(String),
    Mock(Arc<Mutex<MockState>>),
}

pub struct SmartSocket {
    mode: SocketMode,
}

impl SmartSocket {
    /// Connect to a real TCP simulator at `addr` (e.g. "127.0.0.1:7878").
    pub fn new(addr: impl Into<String>) -> Self {
        SmartSocket { mode: SocketMode::Tcp(addr.into()) }
    }

    /// In-memory mock — no network needed. For tests.
    pub fn new_mock(is_on: bool, power: f32) -> Self {
        SmartSocket {
            mode: SocketMode::Mock(Arc::new(Mutex::new(MockState { is_on, power }))),
        }
    }
}
```

Private async helper — opens a connection, writes 1 command byte, reads N response bytes back:

```rust
impl SmartSocket {
    async fn send_command(&self, cmd: u8, response_len: usize) -> Result<Vec<u8>, std::io::Error> {
        let SocketMode::Tcp(addr) = &self.mode else {
            unreachable!("send_command is only called in Tcp mode")
        };
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(&[cmd]).await?;

        let mut buf = vec![0u8; response_len];
        stream.read_exact(&mut buf).await?;
        Ok(buf)
    }
}
```

`read_exact` blocks until all `response_len` bytes arrive — no partial reads, no manual loop.

Public async API — all return `Result` because network calls can fail. The caller decides what to do with an `Err`:

```rust
impl SmartSocket {
    pub async fn is_on(&self) -> Result<bool, std::io::Error> {
        match &self.mode {
            SocketMode::Tcp(_) => {
                let resp = self.send_command(CMD_IS_ON, 1).await?;
                Ok(resp[0] == RESP_ON)
            }
            SocketMode::Mock(state) => Ok(state.lock().unwrap().is_on),
        }
    }

    pub async fn get_current_power(&self) -> Result<f32, std::io::Error> {
        match &self.mode {
            SocketMode::Tcp(_) => {
                let resp = self.send_command(CMD_POWER, 4).await?;
                // 4 bytes → f32 little-endian (same byte order used by the simulator)
                Ok(f32::from_le_bytes([resp[0], resp[1], resp[2], resp[3]]))
            }
            SocketMode::Mock(state) => Ok(state.lock().unwrap().power),
        }
    }

    pub async fn turn_on(&self) -> Result<(), std::io::Error> {
        match &self.mode {
            SocketMode::Tcp(_) => { self.send_command(CMD_TURN_ON, 1).await?; Ok(()) }
            SocketMode::Mock(state) => {
                let mut s = state.lock().unwrap();
                s.is_on = true;
                s.power = 1500.0;
                Ok(())
            }
        }
    }

    pub async fn turn_off(&self) -> Result<(), std::io::Error> {
        match &self.mode {
            SocketMode::Tcp(_) => { self.send_command(CMD_TURN_OFF, 1).await?; Ok(()) }
            SocketMode::Mock(state) => {
                let mut s = state.lock().unwrap();
                s.is_on = false;
                s.power = 0.0;
                Ok(())
            }
        }
    }
}
```

---

## Step 4 — Async UDP `SmartThermometer` in `src/net/thermometer.rs`

**File:** `src/net/thermometer.rs`

The thermometer binds a UDP socket using tokio, then spawns an async task that loops waiting for datagrams. Each datagram is 4 bytes = one `f32` in little-endian byte order. The task updates a shared `Arc<tokio::sync::RwLock<f32>>`. When the thermometer is dropped, the task is cancelled via an `AbortHandle`.

```rust
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use tokio::task::AbortHandle;

pub struct SmartThermometer {
    current_temp: Arc<RwLock<f32>>,
    abort_handle: Option<AbortHandle>,
    local_addr: Option<SocketAddr>,
}
```

Constructor — must be called from within a tokio runtime (e.g., inside `#[tokio::main]` or `#[tokio::test]`):

```rust
impl SmartThermometer {
    /// Bind a UDP socket on `bind_addr` and start listening for temperature datagrams.
    pub async fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(bind_addr).await?;
        let local_addr = socket.local_addr()?;
        let current_temp = Arc::new(RwLock::new(0.0f32));
        let temp_clone = current_temp.clone();

        let handle = tokio::spawn(async move {
            let mut buf = [0u8; 4]; // exactly 4 bytes per datagram
            loop {
                match socket.recv_from(&mut buf).await {
                    Ok((4, _)) => {
                        // parse the 4-byte little-endian float
                        let temp = f32::from_le_bytes(buf);
                        *temp_clone.write().await = temp;
                    }
                    Ok(_) => {
                        // wrong size — ignore silently
                    }
                    Err(e) => {
                        tracing::warn!("Thermometer recv error: {e}");
                        break;
                    }
                }
            }
        })
        .abort_handle();

        Ok(SmartThermometer {
            current_temp,
            abort_handle: Some(handle),
            local_addr: Some(local_addr),
        })
    }

    /// In-memory mock — no network. For tests.
    pub fn new_mock(temp: f32) -> Self {
        SmartThermometer {
            current_temp: Arc::new(RwLock::new(temp)),
            abort_handle: None,
            local_addr: None,
        }
    }

    pub async fn get_current_temperature(&self) -> f32 {
        *self.current_temp.read().await
    }

    pub fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }
}
```

Cancel the background task on drop (using tokio's `AbortHandle` — cleaner than a stop-flag + thread):

```rust
impl Drop for SmartThermometer {
    fn drop(&mut self) {
        if let Some(handle) = self.abort_handle.take() {
            handle.abort();
        }
    }
}
```

`abort()` sends a cancellation signal to the spawned task. Unlike the previous stop-flag approach, it is instant — no `thread::sleep` needed in tests.

---

## Step 5 — TCP socket simulator in `socket-sim/src/main.rs`

**File:** `socket-sim/src/main.rs`

Standalone binary (`cargo run -p socket-sim -- 127.0.0.1:7878`). Listens for TCP connections, one async task per client, state shared via `Arc<tokio::sync::Mutex<_>>`.

```rust
use rand::Rng;
use smarthouse::net::protocol::*;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

struct SocketState {
    is_on: bool,
    power: f32,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7878".to_string());

    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Socket simulator listening on {addr}");

    let state = Arc::new(Mutex::new(SocketState {
        is_on: true,
        power: 1500.0,
    }));

    loop {
        let (stream, client_addr) = listener.accept().await?;
        tracing::info!("Client connected: {client_addr}");
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, state).await {
                tracing::warn!("Client error: {e}");
            }
        });
    }
}

async fn handle_client(
    mut stream: tokio::net::TcpStream,
    state: Arc<Mutex<SocketState>>,
) -> std::io::Result<()> {
    let mut cmd = [0u8; 1];
    loop {
        let n = stream.read(&mut cmd).await?;
        if n == 0 {
            break; // client disconnected
        }

        let mut s = state.lock().await;
        match cmd[0] {
            CMD_IS_ON => {
                let byte = if s.is_on { RESP_ON } else { RESP_OFF };
                stream.write_all(&[byte]).await?;
            }
            CMD_POWER => {
                // respond with 4-byte little-endian f32
                stream.write_all(&s.power.to_le_bytes()).await?;
            }
            CMD_TURN_ON => {
                s.power = rand::rng().random_range(1000.0..2000.0f32);
                s.is_on = true;
                stream.write_all(&[RESP_OK]).await?;
            }
            CMD_TURN_OFF => {
                s.is_on = false;
                s.power = 0.0;
                stream.write_all(&[RESP_OK]).await?;
            }
            other => {
                tracing::warn!("Unknown command: 0x{other:02x}");
                // don't write anything — client will see an error on read
                break;
            }
        }
    }
    Ok(())
}
```

---

## Step 6 — UDP thermometer simulator in `thermo-sim/src/main.rs`

**File:** `thermo-sim/src/main.rs`

Reads config from a TOML file using `tokio::fs`, then loops sending 4-byte binary temperature datagrams.

```rust
use rand::Rng;
use serde::Deserialize;
use std::time::Duration;
use tokio::net::UdpSocket;

#[derive(Deserialize)]
struct Config {
    target_addr: String,
    period_ms: u64,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/thermo_sim.toml".to_string());

    // tokio::fs for async file reading — consistent with the rest of the async code
    let raw = tokio::fs::read_to_string(&config_path).await
        .unwrap_or_else(|e| panic!("Cannot read config file {config_path}: {e}"));
    let config: Config = toml::from_str(&raw)
        .unwrap_or_else(|e| panic!("Invalid config in {config_path}: {e}"));

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    tracing::info!(
        "Thermo simulator: sending to {} every {}ms (binary f32 LE)",
        config.target_addr, config.period_ms
    );

    loop {
        let temp: f32 = rand::rng().random_range(-30.0..50.0f32);
        // Send exactly 4 bytes — f32 in little-endian byte order
        socket.send_to(&temp.to_le_bytes(), &config.target_addr).await?;
        tracing::debug!("Sent temperature: {temp:.1}°C");
        tokio::time::sleep(Duration::from_millis(config.period_ms)).await;
    }
}
```

**Why `tokio::fs` instead of `std::fs`?** The teacher said to use Tokio wholly. `std::fs::read_to_string` blocks the thread — inside an async runtime this starves other tasks. `tokio::fs` wraps it in a thread-pool call so the async scheduler stays responsive.

---

## Step 7 — Thermometer simulator config file

**File:** `config/thermo_sim.toml`

Create the `config/` directory at the workspace root:

```toml
# Address of the SmartThermometer that listens for UDP datagrams.
target_addr = "127.0.0.1:8888"

# How often to send a temperature reading (milliseconds).
period_ms = 1000
```

---

## Step 8 — Error handling strategy

**No new files — this is a cross-cutting pattern used everywhere.**

The teacher's guidance: **errors bubble up to the caller; the caller decides what to do.**

### In library code (`src/net/`)

Every fallible method returns `Result<T, std::io::Error>`. The library never panics or logs on its own — it just propagates:

```rust
// Good: let the caller handle it
pub async fn is_on(&self) -> Result<bool, std::io::Error> { ... }

// Bad: swallowing the error in the library
pub async fn is_on(&self) -> bool {
    self.send_command(...).await.unwrap_or(false)  // ← don't do this
}
```

### In simulator binaries

Simulators can panic on startup errors (bad config, can't bind port) because there's nothing to fall back to:

```rust
let config: Config = toml::from_str(&raw)
    .unwrap_or_else(|e| panic!("Invalid config: {e}"));
```

For per-client errors in the TCP server, log and drop the connection — don't crash the whole server:

```rust
if let Err(e) = handle_client(stream, state).await {
    tracing::warn!("Client error: {e}");  // continue serving other clients
}
```

### In the demo example (application level)

Two strategies, both valid — pick based on severity:

```rust
// Strategy A: panic on first error (fail-fast, good for demos)
let power = socket.get_current_power().await
    .expect("socket_sim must be running for this demo");

// Strategy B: mark device as faulty and continue (robust, good for production)
match socket.is_on().await {
    Ok(state) => println!("Socket: {}", if state { "ON" } else { "OFF" }),
    Err(e) => {
        tracing::warn!("Socket unavailable, skipping: {e}");
        println!("Socket: UNAVAILABLE");
        // continue with other devices
    }
}
```

The demo in Step 10 uses Strategy B so it still produces useful output when a simulator isn't running.

---

## Step 9 — Async unit tests for `SmartSocket`

**File:** `src/net/socket.rs` (add at bottom, inside `#[cfg(test)] mod tests`)

`#[tokio::test]` is the async equivalent of `#[test]` — it spins up a single-threaded tokio runtime just for this test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_socket_is_on_by_default() {
        let s = SmartSocket::new_mock(true, 1500.0);
        assert!(s.is_on().await.unwrap());
    }

    #[tokio::test]
    async fn mock_socket_turn_off() {
        let s = SmartSocket::new_mock(true, 1500.0);
        s.turn_off().await.unwrap();
        assert!(!s.is_on().await.unwrap());
        assert_eq!(s.get_current_power().await.unwrap(), 0.0);
    }

    #[tokio::test]
    async fn mock_socket_turn_on() {
        let s = SmartSocket::new_mock(false, 0.0);
        s.turn_on().await.unwrap();
        assert!(s.is_on().await.unwrap());
    }

    #[tokio::test]
    async fn tcp_socket_returns_error_when_server_is_down() {
        let s = SmartSocket::new("127.0.0.1:19999");
        assert!(s.is_on().await.is_err());
    }

    #[tokio::test]
    async fn tcp_socket_full_flow() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        // Spin up a tiny fake server in the test itself.
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut cmd = [0u8; 1];
            stream.read_exact(&mut cmd).await.unwrap();
            assert_eq!(cmd[0], 0x01); // CMD_IS_ON
            stream.write_all(&[0x01]).await.unwrap(); // RESP_ON
        });

        let s = SmartSocket::new(addr.to_string());
        assert!(s.is_on().await.unwrap());
    }
}
```

---

## Step 10 — Async unit tests for `SmartThermometer`

**File:** `src/net/thermometer.rs` (add at bottom)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::UdpSocket;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn mock_thermometer_returns_initial_temperature() {
        let t = SmartThermometer::new_mock(22.5);
        let got = t.get_current_temperature().await;
        assert!((got - 22.5).abs() < 0.001);
    }

    #[tokio::test]
    async fn udp_thermometer_receives_packets() {
        let t = SmartThermometer::new("127.0.0.1:0").await.unwrap();
        let thermo_addr = t.local_addr().unwrap();

        // Send a 4-byte binary temperature packet.
        let sender = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        sender.send_to(&42.7f32.to_le_bytes(), thermo_addr).await.unwrap();

        // The background task needs a moment to process the packet.
        sleep(Duration::from_millis(100)).await;

        let temp = t.get_current_temperature().await;
        assert!((temp - 42.7).abs() < 0.001, "got {temp}");
    }

    #[tokio::test]
    async fn udp_thermometer_drop_stops_task() {
        // If abort hangs, the test will time out.
        let t = SmartThermometer::new("127.0.0.1:0").await.unwrap();
        drop(t); // should return immediately
    }
}
```

---

## Step 11 — Create the demo example `examples/demo_network.rs`

**File:** `examples/demo_network.rs`

This is an async main. It uses Strategy B error handling: faulty devices are logged and skipped, not panicked.

Run with simulators already started:
```bash
cargo run -p socket-sim -- 127.0.0.1:7878
cargo run -p thermo-sim              # uses config/thermo_sim.toml
cargo run --example demo_network
```

```rust
use smarthouse::net::{SmartSocket, SmartThermometer};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Smart House Network Demo ===\n");

    let socket = SmartSocket::new("127.0.0.1:7878");

    let thermo = match SmartThermometer::new("127.0.0.1:8888").await {
        Ok(t) => Some(t),
        Err(e) => {
            eprintln!("WARNING: Cannot start thermometer listener: {e}");
            None
        }
    };

    // Give the thermometer time to receive its first packet.
    println!("Waiting for first temperature reading...");
    sleep(Duration::from_secs(2)).await;

    println!("\n--- Living Room ---");

    match socket.is_on().await {
        Ok(true)  => println!("Socket: ON"),
        Ok(false) => println!("Socket: OFF"),
        Err(e)    => println!("Socket status UNAVAILABLE: {e} (is socket-sim running?)"),
    }

    match socket.get_current_power().await {
        Ok(power) => println!("Power consumption: {power:.1} W"),
        Err(e)    => println!("Power UNAVAILABLE: {e}"),
    }

    match &thermo {
        Some(t) => println!("Temperature: {:.1}°C", t.get_current_temperature().await),
        None    => println!("Temperature: UNAVAILABLE"),
    }

    println!("\n--- Turning socket OFF ---");
    match socket.turn_off().await {
        Ok(())  => println!("Socket turned off."),
        Err(e)  => println!("Cannot turn off: {e}"),
    }

    match socket.get_current_power().await {
        Ok(power) => println!("Power after turn-off: {power:.1} W"),
        Err(e)    => println!("Power UNAVAILABLE: {e}"),
    }

    println!("\n--- Turning socket ON ---");
    match socket.turn_on().await {
        Ok(())  => println!("Socket turned on."),
        Err(e)  => println!("Cannot turn on: {e}"),
    }

    match socket.get_current_power().await {
        Ok(power) => println!("Power after turn-on: {power:.1} W"),
        Err(e)    => println!("Power UNAVAILABLE: {e}"),
    }
}
```

---

## Step 12 — Final build and lint check

```bash
# Build all workspace members
cargo build --workspace

# Run all tests in all members
cargo test --workspace

# Lint — must be zero warnings across the whole workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Format check
cargo +nightly fmt --all -- --check
```

Common things clippy might flag:
- `unreachable!()` in match arms — restructure to remove if clippy warns
- Missing `use rand::Rng` in simulator (needed for `.random_range()`)
- `tokio::sync::Mutex` vs `std::sync::Mutex` — use tokio's in async context

---

## How to run the full demo

Open three terminals:

```bash
# Terminal 1: start the socket simulator
cargo run -p socket-sim -- 127.0.0.1:7878

# Terminal 2: start the thermometer simulator
cargo run -p thermo-sim

# Terminal 3: run the demo
cargo run --example demo_network
```

Expected output (with simulators running):
```
=== Smart House Network Demo ===

Waiting for first temperature reading...

--- Living Room ---
Socket: ON
Power consumption: 1342.7 W
Temperature: 18.3°C

--- Turning socket OFF ---
Socket turned off.
Power after turn-off: 0.0 W

--- Turning socket ON ---
Socket turned on.
Power after turn-on: 1587.2 W
```

Expected output (without simulators):
```
=== Smart House Network Demo ===

WARNING: Cannot start thermometer listener: Address already in use (os error 48)

Waiting for first temperature reading...

--- Living Room ---
Socket status UNAVAILABLE: Connection refused (os error 61) (is socket-sim running?)
Power UNAVAILABLE: Connection refused (os error 61)
Temperature: UNAVAILABLE
```

---

## Order summary

| Step | What | Touches |
|------|------|---------|
| 0 | Workspace setup — add `[workspace]`, create `socket-sim/` and `thermo-sim/` crates | root `Cargo.toml`, `socket-sim/Cargo.toml`, `thermo-sim/Cargo.toml` |
| 1 | Add tokio to lib | `Cargo.toml` |
| 2 | `net/` skeleton + binary protocol constants | `src/net/mod.rs`, `src/net/protocol.rs`, `src/lib.rs` |
| 3 | Async TCP SmartSocket (binary protocol + mock) | `src/net/socket.rs` |
| 4 | Async UDP SmartThermometer (binary datagrams + AbortHandle) | `src/net/thermometer.rs` |
| 5 | Socket simulator (tokio, binary protocol) | `socket-sim/src/main.rs` |
| 6 | Thermometer simulator (tokio, binary UDP, async config read) | `thermo-sim/src/main.rs` |
| 7 | Thermometer simulator config | `config/thermo_sim.toml` |
| 8 | Error handling patterns (no new files) | docs / demo |
| 9 | Async tests for SmartSocket | bottom of `src/net/socket.rs` |
| 10 | Async tests for SmartThermometer | bottom of `src/net/thermometer.rs` |
| 11 | Async demo example | `examples/demo_network.rs` |
| 12 | Build, test, clippy, fmt (workspace-wide) | — |

Steps 1–4 are the library (client side) and can be done without running any server. Steps 5–7 are the simulators. Steps 9–10 tests don't require running simulators (mock mode + self-contained tokio test servers). Only step 11 (the actual demo) needs all three processes running simultaneously.
