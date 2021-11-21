#[derive(Clone, Copy)]
struct RegisterIndex(usize);

impl RegisterIndex {
    fn validate(value: i64) -> Result<RegisterIndex, String> {
        match value {
            0..=23 => Ok(RegisterIndex(value as _)),
            _ => Err(format!("invalue register index: {}", value)),
        }
    }

    fn value(self) -> usize {
        self.0
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