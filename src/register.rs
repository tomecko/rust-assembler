#[derive(Clone, Copy)]
pub struct RegisterIndex(usize);

impl RegisterIndex {
    pub fn validate(value: i64) -> Result<RegisterIndex, String> {
        match value {
            0..=23 => Ok(RegisterIndex(value as _)),
            _ => Err(format!("invalue register index: {}", value)),
        }
    }

    pub fn value(self) -> usize {
        self.0
    }
}

pub trait Register {
    fn read(&self) -> i64;
    fn write(&mut self, val: i64);
}

#[derive(Clone, Copy)]
pub struct GeneralPurposeRegister {
    pub value: i64,
}

impl Register for GeneralPurposeRegister {
    fn read(&self) -> i64 {
        self.value
    }
    fn write(&mut self, val: i64) {
        self.value = val;
    }
}

pub struct ProgramCounter {
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
