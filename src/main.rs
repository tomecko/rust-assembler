// Single responsibility rule

// Result
// enum Result<T, E> {
//   Err(E),
//   Ok(T)
// }

#[derive(Debug, PartialEq)]
enum Mnemonic {
    Load,
    Mov,
    Add,
    Neg,
    Mul,
    Rev,
    Lea,
    Movgz,
    Movz,
}

impl Mnemonic {
  fn parse(val: &str) -> Result<Self, String> {
     let res = match val.to_lowercase().as_str() {
       "load" =>Mnemonic::Load,
       "mov" => Mnemonic::Mov,
       "add" => Mnemonic::Add,
       "neg" => Mnemonic::Neg,
       "mul" => Mnemonic::Mul,
       "lea" => Mnemonic::Lea,
       "rev" => Mnemonic::Rev,
       "movgz" => Mnemonic::Movgz,
       "movz" => Mnemonic::Movz,
       _ => return Err(format!("unknown mnemonic {} :(", val)),
     };
     Ok(res)
  }
}

#[derive(Debug, PartialEq)]
struct Command {
  mnemonic: Mnemonic,
  args: Vec<i64>,
}

// std::num::ParseIntError

impl Command {
  fn parse(val: &str) -> Result<Self, String> {
    let mut split_val = val.split_whitespace(); // `impl Iterator<Item = &str>`
    let mnemonic = Mnemonic::parse(split_val.next().ok_or("missing mnemonic")?)?;
    let args: Vec<i64> = split_val
      .map(|x| x
        .parse()
        .map_err(|x| format!("unsupported arg {}", x))
      )
      .collect::<Result<_, _>>()?;

    Ok(Self {
      mnemonic,
      args,
    })
  }
}

fn main() {
    println!("{:?}", Command::parse("load 1 2"));
    println!("{:?}", Command::parse("read 1 2"));
    println!("{:?}", Command::parse(""));
    println!("{:?}", Command::parse("load foo bar"));
}

#[cfg(test)]
mod tests {
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
  fn load() {
    assert_eq!(Command::parse("load 1 2").unwrap(), Command {
      mnemonic: Mnemonic::Load,
      args: vec![1, 2],
    });
  }

  // PartialEq: a == b
  // Eq:
  // * a == a
  // * a == b
  // * a == b <=> b == a
  // * a == b and b == c <=> a == c

  // NaN != NaN
}