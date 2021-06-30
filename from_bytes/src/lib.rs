use std::mem::size_of;
use duplicate::duplicate;

pub trait StructFromBytes<T: PackedSize = Self>: PackedSize {
    fn from_bytes(slice: &[u8], offset: usize) -> std::io::Result<Box<Self>>;
}


pub trait PackedSize {
    fn packed_size() -> usize;
}

#[duplicate(
    int_type;
    [u8];
    [u16];
    [u32];
    [u64];
    [u128];
    [i8];
    [i16];
    [i32];
    [i64];
    [i128];
)]
impl PackedSize for int_type { fn packed_size() -> usize { return size_of::<int_type>(); } }

impl PackedSize for [u8;8] { fn packed_size() -> usize { return 8 * size_of::<u8>(); } }
impl PackedSize for [u16;4] { fn packed_size() -> usize { return 4 * size_of::<u16>(); } }
impl PackedSize for [u16;10] { fn packed_size() -> usize { return 10 * size_of::<u16>(); } }