#![allow(dead_code)]

use std::fs::File;
use std::io::Read;

mod command;
mod error;
mod io;
mod machine;
mod mnemonic;
mod operation;
mod register;

use command::Command;
use error::Error;
use io::{StdIO, IO};
use machine::Machine;

fn execute<T: IO>(machine: &mut Machine<T>, program: &str) -> Result<(), Error> {
    let commands: Vec<_> = program
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(Command::parse)
        .filter_map(|x| match x {
          Ok(None) => None,
          Ok(Some(val)) => Some(Ok(val)),
          Err(val) => Some(Err(val)),
        })
        .collect::<Result<_, _>>()?;
    machine.execute(commands)
}

fn read_program_file() -> Result<String, Error> {
    let mut file = File::open("programs/example1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), Error> {
    let program = read_program_file()?;

    let mut io = StdIO;
    let mut machine = Machine::new(&mut io);

    execute(&mut machine, &program)?;

    Ok(())
}

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

    #[test]
    fn pc_program() {
        const EXAMPLE_PROGRAM: &str = "
load 2 14
load 100 0
out 0
";
        test_program(EXAMPLE_PROGRAM, &[], &[0]);
    }

    #[test]
    fn comment_at_the_end() {
        const EXAMPLE_PROGRAM: &str = "
load 1 0 // comment that will be ignored
out 0
";
        test_program(EXAMPLE_PROGRAM, &[], &[1]);
    }

    #[test]
    fn comment_full_line() {
        const EXAMPLE_PROGRAM: &str = "
load 1 0
// comment that will be ignored
out 0
";
        test_program(EXAMPLE_PROGRAM, &[], &[1]);
    }
}
