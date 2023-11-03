use crate::impl_as_bytes;

impl_as_bytes!(i8, Infallible, 1);
impl_as_bytes!(u8, Infallible, 1);

impl_as_bytes!(i16, Infallible, 2);
impl_as_bytes!(u16, Infallible, 2);

impl_as_bytes!(i32, Infallible, 4);
impl_as_bytes!(u32, Infallible, 4);
impl_as_bytes!(f32, Infallible, 4);

impl_as_bytes!(i64, Infallible, 8);
impl_as_bytes!(u64, Infallible, 8);
impl_as_bytes!(f64, Infallible, 8);

impl_as_bytes!(i128, Infallible, 16);
impl_as_bytes!(u128, Infallible, 16);
