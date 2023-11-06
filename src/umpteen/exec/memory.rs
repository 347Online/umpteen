use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::{
    error::{CompilerError, MemoryError},
    repr::value::Value,
};

#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Narrow,
    Wide,
}

impl AddressMode {
    pub const fn max(&self) -> usize {
        match self {
            AddressMode::Narrow => u8::MAX as usize,
            AddressMode::Wide => u16::MAX as usize,
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Byte(u8),
    Word(u16),
}

impl Address {
    pub const fn read(&self) -> usize {
        match self {
            Address::Byte(byte) => *byte as usize,
            Address::Word(word) => *word as usize,
        }
    }

    pub const fn size(&self) -> usize {
        match self {
            Address::Byte(_) => 1,
            Address::Word(_) => 2,
        }
    }

    pub const fn mode(&self) -> AddressMode {
        match self {
            Address::Byte(_) => AddressMode::Narrow,
            Address::Word(_) => AddressMode::Wide,
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", self.read())
    }
}

impl<const N: usize> AsBytes<N> for Address {
    type Error = CompilerError;

    fn to_bytes(self) -> [u8; N] {
        todo!()
    }

    fn try_from_bytes(bytes: [u8; N]) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Memory<'m> {
    values: Vec<Option<Value>>,
    names: HashMap<&'m str, usize>,
}

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

#[test]
fn some_fn() {
    println!("{}", Address::Byte(255))
}
