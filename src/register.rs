use anyhow::Error as AnyError;

use crate::error::Error;

#[derive(Clone, Copy)]
pub struct RegisterIndex(usize);

impl RegisterIndex {
    // this fn can't be const function
    pub fn validate(value: i64) -> Result<RegisterIndex, AnyError> {
        match value {
            0..=23 => Ok(RegisterIndex(value as _)),
            _ => Err(Error::InvalidRegisterIndex(value).into()),
        }
    }

    pub const fn value(self) -> usize {
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
