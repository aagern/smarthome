macro_rules! hi_three {
    ($num:expr, $text:expr) => {
        for _ in 0..$num {
            println!("{}", $text);
        }
    };
}

fn main() {
    hi_three!(5, "Hello!");
}
