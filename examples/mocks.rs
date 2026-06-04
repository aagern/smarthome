#[cfg(feature = "debug")]
fn get_data() -> String {
    "мулька".to_string()
}

#[cfg(not(feature = "debug"))]
fn get_data() -> String {
    "реальная темка".to_string()
}

fn main() {
    println!("{}", get_data());
}
