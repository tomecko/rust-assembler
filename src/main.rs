mod command;
mod mnemonic;

use command::Command;

fn main() {
    println!("{:?}", Command::parse("load 1 2"));
    println!("{:?}", Command::parse("read 1 2"));
    println!("{:?}", Command::parse(""));
    println!("{:?}", Command::parse("load foo bar"));
}
