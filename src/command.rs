use anyhow::Error as AnyError;

use crate::error::Error;
use crate::mnemonic::Mnemonic;

const COMMENT_START: &str = "//";

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub mnemonic: Mnemonic,
    pub args: Vec<i64>,
}

impl Command {
    pub fn parse(val: &str) -> Result<Option<Self>, AnyError> {
        let val_without_comment = val.split(COMMENT_START).next().ok_or(Error::MissingInput)?;
        if val_without_comment.is_empty() {
            return Ok(None);
        }
        let mut split_val = val_without_comment.split_whitespace(); // `impl Iterator<Item = &str>`
        let mnemonic = Mnemonic::parse(split_val.next().ok_or(Error::MissingMnemonic)?)?;
        let args: Vec<i64> = split_val.map(|x| x.parse()).collect::<Result<_, _>>()?;

        Ok(Some(Self { mnemonic, args }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod general {
        use super::*;

        #[test]
        fn empty_line() {
            assert_eq!(
                Command::parse("").unwrap(),
                None
            );
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
                Some(Command {
                    mnemonic: Mnemonic::Load,
                    args: vec![1, 2],
                })
            );
        }
    }
}
