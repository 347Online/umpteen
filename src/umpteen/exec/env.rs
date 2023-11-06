use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    error::{MemoryError, UmpteenError},
    repr::value::Value,
};

#[derive(Debug, Default)]
pub struct Memory<'m> {
    values: Vec<Option<Value>>,
    names: HashMap<&'m str, usize>,
}

pub type Stack = Vec<Value>;

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

    fn offset(&self) -> usize {
        self.values.len()
    }

    fn retrieve(&self, name: &str) -> Result<usize, MemoryError> {
        let addr = *self
            .names
            .get(name)
            .unwrap_or_else(|| panic!("unknown identifier {}", name)); // TODO: Create an error variant instead of expect

        Ok(addr)
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
