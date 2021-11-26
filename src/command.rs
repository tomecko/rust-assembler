use crate::mnemonic::Mnemonic;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub mnemonic: Mnemonic,
    pub args: Vec<i64>,
}

impl Command {
    pub fn parse(val: &str) -> Result<Self, String> {
        let mut split_val = val.split_whitespace(); // `impl Iterator<Item = &str>`
        let mnemonic = Mnemonic::parse(split_val.next().ok_or("missing mnemonic")?)?;
        let args: Vec<i64> = split_val
            .map(|x| x.parse().map_err(|x| format!("unsupported arg {}", x)))
            .collect::<Result<_, _>>()?;

        Ok(Self { mnemonic, args })
    }
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
    }
}