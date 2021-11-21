pub struct Machine {
    registers: [Box<dyn Register>; 14],
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
        for command in commands {}
    }
}

// fn execute_command(command: Command) {
//     match command {
//         //    Command { mnemonic: Mnemonic::Load , args } => get_register(args[1]).write(args[0]),
//         _ => todo!(),
//     }
// }