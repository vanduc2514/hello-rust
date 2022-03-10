use ferris_says::say;
use std::io::{stdout, BufWriter};
use String;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans! I'm Duc!!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("{}", "~".repeat(20)); // Why does this print before ferris_say ?
}
