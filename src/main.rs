mod command;
mod mnemonic;

use command::Command;

fn main() {
    println!("{:?}", Command::parse("load 1 2"));
    println!("{:?}", Command::parse("read 1 2"));
    println!("{:?}", Command::parse(""));
    println!("{:?}", Command::parse("load foo bar"));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod general {
        use super::*;

        #[test]
        fn empty_line() {
            Command::parse("").unwrap_err();
        }

        #[test]
        fn unknown_mnemonic() {
            Command::parse("abc_unknown 1 2").unwrap_err();
        }

        #[test]
        fn invalid_args() {
            Command::parse("load a b").unwrap_err();
        }
    }

    mod load {
        use super::*;

        #[test]
        fn load_command() {
            assert_eq!(
                Command::parse("load 1 2").unwrap(),
                Command {
                    mnemonic: Mnemonic::Load,
                    args: vec![1, 2],
                }
            );
        }

        #[test]
        fn load_register() {
            execute(Command::parse("load 1 2"));
            assert_eq!(get_register(2).read(), 1);
        }
    }
}
