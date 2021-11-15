// use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mnemonic {
    Load,
    Mov,
    Add,
    Neg,
    Mul,
    Lea,
    Movgz,
    Movz,
}

impl Mnemonic {
  fn parse(val: &str) -> Result<Self, String> {
     let res = match val.to_lowercase().as_str() {
       "load" => Mnemonic::Load,
       "mov" => Mnemonic::Mov,
       "add" => Mnemonic::Add,
       "neg" => Mnemonic::Neg,
       "mul" => Mnemonic::Mul,
       "lea" => Mnemonic::Lea,
       "movgz" => Mnemonic::Movgz,
       "movz" => Mnemonic::Movz,
       _ => return Err(format!("unknown mnemonic {} :(", val)),
     };
     Ok(res)
  }
}

#[derive(Debug, PartialEq, Clone)]
struct Command {
  mnemonic: Mnemonic,
  args: Vec<i64>,
}

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

#[derive(Clone, Copy)]
struct RegisterIndex(usize);

impl RegisterIndex {
  fn validate(value: i64) -> Result<RegisterIndex, String> {
    match value {
      0..=23 => Ok(RegisterIndex(value as _)),
      _ => Err(format!("value {} outside supported scope of register index", value)),
    }
  }

  fn value(self) -> usize {
    self.0
  }
}

enum Operation {
  Load(i64, RegisterIndex),
  Mov(RegisterIndex, RegisterIndex),
  Add(RegisterIndex, RegisterIndex, RegisterIndex),
  Neg(RegisterIndex, RegisterIndex),
  Mul(RegisterIndex, RegisterIndex, RegisterIndex),
  Lea(RegisterIndex, i64, RegisterIndex, RegisterIndex),
  Movgz(RegisterIndex, RegisterIndex, RegisterIndex),
  Movz(RegisterIndex, RegisterIndex, RegisterIndex),
}

impl Operation {
  fn validate(command: Command) -> Result<Self, String> {
    let Command { mnemonic, args } = command;
    
    match mnemonic {
      Mnemonic::Load => {
        Self::check_args_size(&args, 2)?;
        Ok(
          Self::Load(args[0],
          RegisterIndex::validate(args[1])?)
        )
      }
      Mnemonic::Mov => {
        Self::check_args_size(&args, 2)?;
        Ok(Self::Mov(
          RegisterIndex::validate(args[0])?,
          RegisterIndex::validate(args[1])?,
        ))
      }
      _ => todo!(),
    }
  }

  fn execute(&self, machine: &mut Machine) {
    use Operation::*;

    match self {
      Load(imm, reg) => machine.registers[reg.value()].write(*imm),
      Mov(from_reg, to_reg) =>
        machine.registers[to_reg.value()].write(machine.registers[from_reg.value()].read()),
      _ => todo!(),
      // TODO
    }
  }

  fn check_args_size(args: &[i64], expected: usize) -> Result<(), String> {
    if args.len() == expected {
      Ok(())
    } else {
      Err(format!("wrong number of arguments {}, expected: {}", args.len(), expected))
    }
  }
}

trait Register {
  fn read(&self) -> i64;
  fn write(&mut self, val: i64);
}

#[derive(Clone, Copy)]
struct GeneralPurposeRegister {
  value: i64,
}

impl Register for GeneralPurposeRegister {
  fn read(&self) -> i64 {
    self.value
  }
  fn write(&mut self, val: i64) {
    self.value = val;
  }
}

struct ProgramCounter {
  value: i64,
}

impl Register for ProgramCounter {
  fn read(&self) -> i64 {
    todo!()
  }
  fn write(&mut self, val: i64) {
    println!("{}", val);
    todo!()
  }
}

/* fn get_register(n: i64) -> Result<impl Register, String> {
  let registers: HashMap<i64, _> = HashMap::new();
  if let Some(existingRegister) = registers.get(&n) {
    Ok(existingRegister)
  } else {
    let newRegister = match n {
      0..=13 => Some(GeneralPurposeRegister { value: n }),
      _ => None
    };
    if let Some(register) = newRegister {
      registers.insert(n, register);
      Ok(&register)
    } else {
      Err(format!("unable to get register for n = {}", n))
    }
  }
} */

struct Machine {
  registers: [Box<dyn Register>; 14],
}

impl Machine {
  fn new() -> Self {
    Machine {
      registers: [
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
        Box::new(GeneralPurposeRegister { value: 0 }),
      ]
    }
  }

  fn execute(commands: Vec<Command>) {
    for command in commands {
      
    }
  }
}

fn execute_command(command: Command) {
  match command {
//    Command { mnemonic: Mnemonic::Load , args } => get_register(args[1]).write(args[0]),
    _ => todo!(),
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
      assert_eq!(Command::parse("load 1 2").unwrap(), Command {
        mnemonic: Mnemonic::Load,
        args: vec![1, 2],
      });
    }

    #[test]
    fn load_register() {
      execute(Command::parse("load 1 2"));
      assert_eq!(get_register(2).read(), 1);
    }
  }

}