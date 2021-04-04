use packed_size::*;

pub trait StructFromBytes<T: PackedSize = Self>: PackedSize {
    fn from_bytes(slice: &[u8], offset: usize) -> std::io::Result<Box<Self>>;
}