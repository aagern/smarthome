# Refactoring Plan: Legacy → HashMap (DZ2)

This plan walks through every change needed to complete the homework assignment.
Each step is small and can be compiled and tested on its own.

---

## The Big Picture

Right now the project has two worlds living side by side:

- **`src/legacy/`** — the old world: `NonEmptyVec<T>`, rooms and devices accessed by number index (`RoomId(1)`, `DeviceId(2)`)
- **`src/new/`** — the new world (half-built): `HashMap<String, T>`, rooms and devices accessed by name (`"bedroom"`, `"socket1"`)

`src/lib.rs` line 112 currently says `pub use legacy::{House, Room}`, which means the whole project uses the old world. When you're done, that line will say `pub use new::{House, Room}` instead.

---

## Step 1 — Expand `InputError` with named variants

**File:** `src/lib.rs`

Right now `InputError` has:
```rust
DeviceNotFound,
RoomNotFound,
DataEmpty,
```

The task says `get_device(room_name, device_name)` must return an error that tells you *what* was missing. Change the variants to carry the name that wasn't found:

```rust
#[error("Device not found: {0}")]
DeviceNotFound(String),

#[error("Room not found: {0}")]
RoomNotFound(String),
```

`DataEmpty` can stay as-is — it doesn't need a name.

Think of it like this: instead of "something wasn't found", the error now says "the room named 'bathroom' wasn't found". Much more useful.

> **Why now?** Everything else in the plan depends on this type. Do it first so the compiler guides you through all the places that need updating.

---

## Step 2 — Allow empty `Room` and `House` in `src/new/`

**Files:** `src/new/room.rs`, `src/new/house.rs`

The task says empty rooms and houses are now okay — you might create an empty house and add rooms later.

In `src/new/room.rs`: remove the `if devices.is_empty()` check from `try_from_vec`, and also add a plain `Room::new_empty()` constructor that returns an empty room.

In `src/new/house.rs`: similarly add `House::new_empty()`.

This is the opposite of what `NonEmptyVec` did. Before: "you must give me at least one thing". Now: "fine, start empty, add things whenever".

---

## Step 3 — Add `From` conversions for devices

**File:** `src/lib.rs` (or a new small file `src/device.rs` that you `mod` from `lib.rs`)

The task wants:
```rust
SmartSocket    → SmartDevice
SmartThermometer → SmartDevice
```

This is just:
```rust
impl From<SmartSocket> for SmartDevice {
    fn from(s: SmartSocket) -> Self { SmartDevice::Socket(s) }
}

impl From<SmartThermometer> for SmartDevice {
    fn from(t: SmartThermometer) -> Self { SmartDevice::Thermometer(t) }
}
```

Why bother? The macro in Step 5 will use these conversions internally so the caller doesn't have to write `SmartDevice::Socket(...)` by hand.

---

## Step 4 — Add `get_device` shortcut on `House` (the combined lookup)

**File:** `src/new/house.rs`

Add this method:
```rust
pub fn get_device(&self, room_name: &str, device_name: &str) -> Result<&SmartDevice, InputError>
```

It looks up the room by name — if missing, returns `InputError::RoomNotFound(room_name.to_string())`. Then looks up the device by name inside that room — if missing, returns `InputError::DeviceNotFound(device_name.to_string())`.

This is like a two-level map lookup with a specific error message at each level.

---

## Step 5 — Write the `room!` macro

**File:** `src/lib.rs` (at the bottom, or in a dedicated `src/macros.rs`)

The macro should let you write:
```rust
let r = room!("socket1" => SmartSocket::default(), "thermo1" => SmartThermometer::default());
```

and it creates a `new::Room` with those two devices, automatically wrapping them in `SmartDevice` via the `From` conversions from Step 3.

A declarative macro (`macro_rules!`) is enough here. The pattern is:
- accept zero or more `"key" => expr` pairs
- for each pair, call `.into()` on the value (that's where the `From` impl kicks in)
- collect them into a `HashMap` and construct `Room`

Mark it `#[macro_export]` so it's available to users of the library.

---

## Step 6 — Add the `Report` trait

**File:** new file `src/report.rs`, declared as `mod report; pub use report::Report;` in `lib.rs`

```rust
pub trait Report {
    fn report(&self) -> String;
}
```

Then implement it on:
- `SmartDevice` — already has `Display`; `report()` can return `self.to_string()`
- `new::Room` — list all devices with their names
- `new::House` — list all rooms with their names

This is just making "print me" an official contract. Any type that implements `Report` can be passed to the demo function in `main.rs`.

---

## Step 7 — Move the tests to their own module file

**Currently:** all tests live in `src/lib.rs` in a `#[cfg(test)] mod tests` block.

**Problem:** those tests use the *legacy* types. After you flip the `pub use` switch they will fail.

**Fix:**

1. Create `src/tests.rs` (or `src/tests/mod.rs` for a folder).
2. Move the existing tests there unchanged — they now live in `mod tests` inside that file. Add `#[cfg(test)] mod tests;` to `lib.rs`.
3. Create `src/new/tests.rs` (or `src/new/room.rs` and `src/new/house.rs` each get their own `#[cfg(test)] mod tests` block at the bottom).

The old tests now explicitly import from `legacy::`:
```rust
use crate::legacy::{House, Room};
```
…so they still pass even after the `pub use` switch. They're a safety net, not something to delete.

---

## Step 8 — Write new tests for `src/new/`

**Files:** bottom of `src/new/room.rs` and `src/new/house.rs`, or a separate `src/new/tests.rs`

New tests to add (per the answers in `questions_dz2.md`):

| What to test | How |
|---|---|
| `add_device` / `remove_device` | add one, check count; remove it, check count |
| `add_room` / `remove_room` | same idea on `House` |
| `get_device` on House (two-level lookup) | valid path returns `Ok`; bad room name returns `RoomNotFound`; bad device name returns `DeviceNotFound` |
| `room!` macro | creates room with correct device count |
| `Report` trait | `report()` returns a non-empty string |
| Empty room/house | creating empty is allowed, adding then removing works |

No need to test `From` conversions — the compiler checks those for you.

---

## Step 9 — Update `src/main.rs` to use the new API

**File:** `src/main.rs`

The current `main.rs` uses `RoomId` and `DeviceId` (index-based). Replace the demo with one that:

1. Creates rooms using the `room!` macro
2. Builds a house using `House::new_empty()` + `add_room`
3. Shows a report using a helper function `fn print_report(r: &impl Report)`
4. Demonstrates removing a device and a room
5. Demonstrates the error path: try to get a device from a non-existent room, print the error

---

## Step 10 — Flip the switch and clean up

**File:** `src/lib.rs`, lines 112–113

Change:
```rust
pub use legacy::{House, Room};
// pub use new::{House, Room};
```
to:
```rust
// pub use legacy::{House, Room};
pub use new::{House, Room};
```

Now run:
```
cargo build
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo +nightly fmt -- --check
```

Fix anything the compiler or clippy complains about. Common things to expect:
- `RoomId` / `DeviceId` no longer needed in `main.rs` — remove those imports
- `NonEmptyVec` no longer in scope — that's fine, `legacy` still compiles internally
- `#[allow(unused)]` at the top of `src/new/house.rs` — remove it once the code is used

---

## Step 11 — Update `examples/demo_hashmap.rs`

**File:** `examples/demo_hashmap.rs`

This example is almost correct already. Update it to:
- Use the `room!` macro instead of `Room::try_from_vec`
- Call `print_report` (or just `println!("{}", house.report())`) to show the `Report` trait working

Run with: `cargo run --example demo_hashmap`

---

## Order summary

| Step | What | Touches |
|------|------|---------|
| 1 | Expand `InputError` | `src/lib.rs` |
| 2 | Allow empty collections | `src/new/room.rs`, `src/new/house.rs` |
| 3 | `From` conversions | `src/lib.rs` |
| 4 | `House::get_device` two-level lookup | `src/new/house.rs` |
| 5 | `room!` macro | `src/lib.rs` |
| 6 | `Report` trait + impls | new `src/report.rs` + `src/new/*.rs` |
| 7 | Move old tests to their own file | `src/lib.rs` → `src/tests.rs` |
| 8 | Write new tests | `src/new/*.rs` |
| 9 | Rewrite `main.rs` | `src/main.rs` |
| 10 | Flip `pub use` switch + fix errors | `src/lib.rs` |
| 11 | Update example | `examples/demo_hashmap.rs` |

Steps 1–6 can be done while the old code is still active (the `pub use legacy` line is still in place), so nothing breaks until Step 10. Steps 7–8 should be done before Step 10 so the tests are ready when you flip the switch.
