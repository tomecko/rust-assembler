use anyhow::Error as AnyError;

use crate::command::Command;
use crate::error::Error;
use crate::mnemonic::Mnemonic;
use crate::register::RegisterIndex;

#[derive(Debug)]
pub enum Operation {
    Load(i64, RegisterIndex),
    Mov(RegisterIndex, RegisterIndex),
    Add(RegisterIndex, RegisterIndex, RegisterIndex),
    Neg(RegisterIndex, RegisterIndex),
    Mul(RegisterIndex, RegisterIndex, RegisterIndex),
    Lea(RegisterIndex, i64, RegisterIndex, RegisterIndex),
    Movgz(RegisterIndex, RegisterIndex, RegisterIndex),
    Movz(RegisterIndex, RegisterIndex, RegisterIndex),
    In(RegisterIndex),
    Out(RegisterIndex),
}

impl Operation {
    pub fn validate(command: Command) -> Result<Self, AnyError> {
        let Command { mnemonic, args } = command;

        match mnemonic {
            Mnemonic::Load => {
                Self::check_args_size(&args, 2)?;
                Ok(Self::Load(args[0], RegisterIndex::validate(args[1])?))
            }
            Mnemonic::Mov => {
                Self::check_args_size(&args, 2)?;
                Ok(Self::Mov(
                    RegisterIndex::validate(args[0])?,
                    RegisterIndex::validate(args[1])?,
                ))
            }
            Mnemonic::Add => {
                Self::check_args_size(&args, 3)?;
                Ok(Self::Add(
                    RegisterIndex::validate(args[0])?,
                    RegisterIndex::validate(args[1])?,
                    RegisterIndex::validate(args[2])?,
                ))
            }
            Mnemonic::Neg => {
                Self::check_args_size(&args, 2)?;
                Ok(Self::Neg(
                    RegisterIndex::validate(args[0])?,
                    RegisterIndex::validate(args[1])?,
                ))
            }
            Mnemonic::Mul => {
                Self::check_args_size(&args, 3)?;
                Ok(Self::Mul(
                    RegisterIndex::validate(args[0])?,
                    RegisterIndex::validate(args[1])?,
                    RegisterIndex::validate(args[2])?,
                ))
            }
            Mnemonic::Lea => {
                Self::check_args_size(&args, 4)?;
                Ok(Self::Lea(
                    RegisterIndex::validate(args[0])?,
                    args[1],
                    RegisterIndex::validate(args[2])?,
                    RegisterIndex::validate(args[3])?,
                ))
            }
            Mnemonic::Movgz => {
                Self::check_args_size(&args, 3)?;
                Ok(Self::Movgz(
                    RegisterIndex::validate(args[0])?,
                    RegisterIndex::validate(args[1])?,
                    RegisterIndex::validate(args[2])?,
                ))
            }
            Mnemonic::Movz => {
                Self::check_args_size(&args, 3)?;
                Ok(Self::Movz(
                    RegisterIndex::validate(args[0])?,
                    RegisterIndex::validate(args[1])?,
                    RegisterIndex::validate(args[2])?,
                ))
            }
            Mnemonic::In => {
                Self::check_args_size(&args, 1)?;
                Ok(Self::In(RegisterIndex::validate(args[0])?))
            }
            Mnemonic::Out => {
                Self::check_args_size(&args, 1)?;
                Ok(Self::Out(RegisterIndex::validate(args[0])?))
            }
        }
    }

    fn check_args_size(args: &[i64], expected: usize) -> Result<(), AnyError> {
        if args.len() == expected {
            Ok(())
        } else {
            Err(Error::WrongNumberOfArguments {
                received: args.len(),
                expected,
            }
            .into())
        }
    }
}
