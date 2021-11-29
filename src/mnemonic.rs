#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    Load,
    Mov,
    Add,
    Neg,
    Mul,
    Lea,
    Movgz,
    Movz,
    In,
    Out,
}

impl Mnemonic {
    pub fn parse(val: &str) -> Result<Self, String> {
        let res = match val.to_lowercase().as_str() {
            "load" => Mnemonic::Load,
            "mov" => Mnemonic::Mov,
            "add" => Mnemonic::Add,
            "neg" => Mnemonic::Neg,
            "mul" => Mnemonic::Mul,
            "lea" => Mnemonic::Lea,
            "movgz" => Mnemonic::Movgz,
            "movz" => Mnemonic::Movz,
            "in" => Mnemonic::In,
            "out" => Mnemonic::Out,
            _ => return Err(format!("unknown mnemonic {} :(", val)),
        };
        Ok(res)
    }
}
