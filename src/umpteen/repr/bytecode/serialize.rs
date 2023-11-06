use std::convert::Infallible;

pub trait AsBytes<const N: usize>
where
    Self: Sized,
{
    type Error: std::error::Error;

    fn to_bytes(self) -> [u8; N];
    fn try_from_bytes(bytes: [u8; N]) -> Result<Self, Self::Error>;
}

const fn size_of<T>() -> usize {
    std::mem::size_of::<T>()
}

macro_rules! impl_as_bytes {
    ($t:ty, $e:ty) => {
        impl AsBytes<{size_of::<$t>()}> for $t {
            type Error = $e;
            fn to_bytes(self) -> [u8; size_of::<$t>()] {
                self.to_be_bytes()
            }

            fn try_from_bytes(bytes: [u8; {size_of::<$t>()}]) -> Result<Self, Self::Error> {
                Ok(Self::from_be_bytes(bytes))
            }
        }
    };
}

impl_as_bytes!(i8, Infallible);
impl_as_bytes!(i16, Infallible);
impl_as_bytes!(i32, Infallible);
impl_as_bytes!(i64, Infallible);
impl_as_bytes!(i128, Infallible);

impl_as_bytes!(u8, Infallible);
impl_as_bytes!(u16, Infallible);
impl_as_bytes!(u32, Infallible);
impl_as_bytes!(u64, Infallible);
impl_as_bytes!(u128, Infallible);

impl_as_bytes!(f32, Infallible);
impl_as_bytes!(f64, Infallible);
