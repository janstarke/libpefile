pub trait StructFromBytes {
    fn from_bytes(bytes: &[u8]) -> std::io::Result<Box<Self>>;
}