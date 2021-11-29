pub trait IO {
    fn input(&mut self) -> Result<i64, String>;
    fn output(&mut self, value: i64) -> Result<(), String>;
}

pub struct MockIO {
    inputs: Vec<i64>,
    pub outputs: Vec<i64>,
}

impl IO for MockIO {
    fn input(&mut self) -> Result<i64, String> {
        self.inputs
            .pop()
            .ok_or("missing input")
            .map_err(String::from)
    }
    fn output(&mut self, value: i64) -> Result<(), String> {
        self.outputs.push(value);
        Ok(())
    }
}

impl MockIO {
    pub fn new(mut inputs: Vec<i64>) -> Self {
        inputs.reverse();
        MockIO {
            // reversing because taking the last item is cheaper
            inputs,
            outputs: Vec::new(),
        }
    }
}
