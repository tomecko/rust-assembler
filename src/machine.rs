use crate::command::Command;
use crate::operation::Operation;
use crate::register::{Register,GeneralPurposeRegister};


pub struct Machine {
    pub registers: [Box<dyn Register>; 14],
}

impl Machine {
    pub fn new() -> Self {
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
        }
    }

    pub fn execute(commands: Vec<Command>) {
        // Vec<Command> -> Vec<Operation>

        // iter: impl Iterator<Item = Result<Operation, String>>
        let _iter: Result<Vec<Operation>, String> = commands
            .into_iter()
            .map(|c| Operation::validate(c))
            .collect();

        todo!()
    }
}

// fn execute_command(command: Command) {
//     match command {
//         //    Command { mnemonic: Mnemonic::Load , args } => get_register(args[1]).write(args[0]),
//         _ => todo!(),
//     }
// }
