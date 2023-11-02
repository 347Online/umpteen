pub mod chunk;
pub mod error;
pub mod instr;
pub mod token;
pub mod value;
pub mod expr;

pub trait AsBytes<const N: usize> {
  fn to_bytes(self) -> [u8; N];
  fn from_bytes(bytes: [u8; N]) -> Self;
}