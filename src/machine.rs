use crate::command::Command;
use crate::io::IO;
use crate::operation::Operation;
use crate::register::{GeneralPurposeRegister, Register, RegisterIndex};

pub struct Machine<'io, T> {
    registers: [Box<dyn Register>; 14],
    io: &'io mut T,
}

impl<'io, T> Machine<'io, T> {
    pub fn new(io: &'io mut T) -> Self {
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
            ],
            io,
        }
    }
}

impl<'io, T> Machine<'io, T>
where
    T: IO,
{
    pub fn execute(&mut self, commands: Vec<Command>) -> Result<(), String>
    where
        T: IO,
    {
        let ops: Vec<_> = commands
            .into_iter()
            .map(Operation::validate)
            .collect::<Result<_, _>>()?;
        for op in ops {
            self.execute_operation(&op)?;
        }
        Ok(())
    }

    fn execute_operation(&mut self, operation: &Operation) -> Result<(), String> {
        use Operation::*;

        match operation {
            Load(imm, reg) => self.registers[reg.value()].write(*imm),
            Mov(from_reg, to_reg) => {
                self.registers[to_reg.value()].write(self.registers[from_reg.value()].read())
            }
            Add(from_reg_1, from_reg_2, to_reg) => {
                let sum = self.registers[from_reg_1.value()].read()
                    + self.registers[from_reg_2.value()].read();
                self.registers[to_reg.value()].write(sum)
            }
            In(reg) => self.input(*reg)?,
            Out(reg) => self.output(*reg)?,
            _ => todo!(),
        }

        Ok(())
    }

    pub fn input(&mut self, reg: RegisterIndex) -> Result<(), String> {
        let value = self.io.input()?;
        self.registers[reg.value()].write(value);
        Ok(())
    }

    pub fn output(&mut self, reg: RegisterIndex) -> Result<(), String> {
        self.io.output(self.registers[reg.value()].read())
    }
}
