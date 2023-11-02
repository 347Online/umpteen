use std::{arch::aarch64::vaba_s16, collections::HashMap};

use crate::{
    repr::{error::Error, value::Value},
    Result,
};

#[derive(Debug, Default)]
pub struct Environment {
    data: HashMap<String, Value>,
}

impl Environment {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self { data }
    }
    pub fn get(&self, name: &str) -> Value {
        self.data.get(name).into()
    }

    pub fn declare(&mut self, name: &str) -> Result<()> {
        match self.data.insert(name.to_string(), Value::default()) {
            Some(_) => Err(Error::IllegalDeclare),
            None => Ok(()),
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        self.data.insert(name.to_string(), value);
    }
}
