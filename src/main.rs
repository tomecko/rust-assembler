#![allow(dead_code)]

mod command;
mod io;
mod machine;
mod mnemonic;
mod operation;
mod register;

use command::Command;
use io::IO;
use machine::Machine;

fn execute<T: IO>(machine: &mut Machine<T>, program: &str) -> Result<(), String> {
    // Vec<Result<Command, String>>
    let commands: Vec<_> = program
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(Command::parse)
        .collect::<Result<_, _>>()?;
    machine.execute(commands)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::io::MockIO;

    fn test_program(program: &str, inputs: &[i64], outputs: &[i64]) {
        // Given
        let mut mock_io = MockIO::new(inputs.to_owned());
        let mut machine = Machine::new(&mut mock_io);

        // When
        execute(&mut machine, program).unwrap();

        // Then
        assert_eq!(mock_io.outputs, outputs);
    }

    #[test]
    fn simple_program() {
        const EXAMPLE_PROGRAM: &str = "
out 0
load 10 0
mov 0 1
out 0
out 1
";

        test_program(EXAMPLE_PROGRAM, &[], &[0, 10, 10]);
    }

    #[test]
    fn add_program() {
        const EXAMPLE_PROGRAM: &str = "
load 10 0
load 20 1
add 0 1 2
out 0
out 1
out 2
";
        test_program(EXAMPLE_PROGRAM, &[], &[10, 20, 30]);
    }
}
