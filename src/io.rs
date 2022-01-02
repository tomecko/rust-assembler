use std::io;

use anyhow::anyhow;
use std::io::BufRead;

use crate::error::Error;

pub trait IO {
    fn input(&mut self) -> Result<i64, Error>;
    // returns Result type because in general it can fail (e.g. due to networking issue)
    fn output(&mut self, value: i64) -> Result<(), Error>;
}

pub struct StdIO;

impl IO for StdIO {
    fn input(&mut self) -> Result<i64, Error> {
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        let input = input.trim().parse::<i64>().map_err(|err| anyhow!(err))?;
        Ok(input)
    }
    fn output(&mut self, value: i64) -> Result<(), Error> {
        println!("{}", value);
        Ok(())
    }
}

pub struct MockIO {
    inputs: Vec<i64>,
    pub outputs: Vec<i64>,
}

impl IO for MockIO {
    fn input(&mut self) -> Result<i64, Error> {
        self.inputs.pop().ok_or(Error::MissingInput.into())
    }
    fn output(&mut self, value: i64) -> Result<(), Error> {
        self.outputs.push(value);
        Ok(())
    }
}

impl MockIO {
    pub fn new(mut inputs: Vec<i64>) -> Self {
        // reversing because taking the last item is cheaper
        inputs.reverse();
        MockIO {
            inputs,
            outputs: Vec::new(),
        }
    }
}
