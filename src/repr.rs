use std::convert::Infallible;

pub mod chunk;
pub mod error;
pub mod expr;
pub mod instr;
pub mod object;
pub mod token;
pub mod value;

pub trait AsBytes<const N: usize>
where
    Self: Sized,
{
    type Error;

    fn to_bytes(self) -> [u8; N];
    fn try_from_bytes(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

macro_rules! impl_as_bytes {
    ($t:ty, $e:ty, $n:expr) => {
        impl AsBytes<$n> for $t {
            type Error = $e;
            fn to_bytes(self) -> [u8; $n] {
                self.to_be_bytes()
            }

            fn try_from_bytes(bytes: [u8; $n]) -> Result<Self, Self::Error> {
                Ok(Self::from_be_bytes(bytes))
            }
        }
    };
}

impl_as_bytes!(i8, Infallible, 1);
impl_as_bytes!(i16, Infallible, 2);
impl_as_bytes!(i32, Infallible, 4);
impl_as_bytes!(i64, Infallible, 8);
impl_as_bytes!(i128, Infallible, 16);

impl_as_bytes!(u8, Infallible, 1);
impl_as_bytes!(u16, Infallible, 2);
impl_as_bytes!(u32, Infallible, 4);
impl_as_bytes!(u64, Infallible, 8);
impl_as_bytes!(u128, Infallible, 16);

impl_as_bytes!(f32, Infallible, 4);
impl_as_bytes!(f64, Infallible, 8);
