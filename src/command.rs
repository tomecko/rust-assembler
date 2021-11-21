use crate::mnemonic::Mnemonic;

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    mnemonic: Mnemonic,
    args: Vec<i64>,
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
