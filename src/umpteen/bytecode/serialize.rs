pub trait AsBytes<const N: usize>
where
    Self: Sized,
{
    type Error;

    fn to_bytes(self) -> [u8; N];
    fn try_from_bytes(bytes: [u8; N]) -> Result<Self, Self::Error>;
}
