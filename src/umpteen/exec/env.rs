use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::{error::MemoryError, repr::value::Value};

#[derive(Debug, Default)]
pub struct Memory<'m> {
    values: Vec<Option<Value>>,
    names: HashMap<&'m str, usize>,
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

impl<'m> Memory<'m> {
    pub fn declare_constant(&mut self, value: Value) -> usize {
        let addr = self.offset();
        self.values.push(Some(value));
        addr
    }

    pub fn declare(&mut self, name: &'m str) -> Result<usize, MemoryError> {
        if self.names.contains_key(name) {
            panic!("variable already declared") // TODO: Create an error variant instead of panic
        } else {
            let addr = self.offset();
            self.values.push(None);
            self.names.insert(name, addr);
            Ok(addr)
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), MemoryError> {
        let addr = self.retrieve(name)?;
        self.values[addr] = Some(value);

        Ok(())
    }

    pub fn get(&self, addr: usize) -> Result<Value, MemoryError> {
        let value = self
            .values
            .get(addr)
            .cloned()
            .flatten()
            .ok_or(MemoryError::InvalidReference(addr))?;

        Ok(value)
    }

    pub fn retrieve(&self, name: &str) -> Result<usize, MemoryError> {
        let addr = *self.names.get(name).unwrap();

        Ok(addr)
    }

    fn offset(&self) -> usize {
        self.values.len()
    }
}

impl<'m> Deref for Memory<'m> {
    type Target = Vec<Option<Value>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'m> DerefMut for Memory<'m> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
