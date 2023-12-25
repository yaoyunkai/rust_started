use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    test_println();
}

fn test_println() {
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
}
