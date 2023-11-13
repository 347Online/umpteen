use std::{collections::HashMap, fmt::Display};

use crate::{error::MemoryError, repr::value::Value};

#[derive(Debug, Default)]
pub struct Memory {
    vars: HashMap<String, Option<Value>>,
}

#[derive(Debug)]
pub enum StackItem {
    Address(usize),
    Value(Value),
}

impl Display for StackItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackItem::Address(addr) => write!(f, "{}", addr),
            StackItem::Value(val) => write!(f, "{}", val),
        }
    }
}

pub type Stack = Vec<StackItem>;

impl Memory {
    pub fn declare(&mut self, name: &str) -> Result<(), MemoryError> {
        if self.vars.contains_key(name) {
            panic!("variable already declared")
        } else {
            self.vars.insert(name.to_string(), None);
        }

        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), MemoryError> {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), Some(value));
        } else {
            Err(MemoryError::NoSuchVariable(name.to_string()))?
        }

        Ok(())
    }

    pub fn get(&mut self, name: &str) -> Result<Value, MemoryError> {
        self.vars
            .get(name)
            .cloned()
            .flatten()
            .ok_or(MemoryError::UninitializedVariableAccess(name.to_owned()))
    }
}
