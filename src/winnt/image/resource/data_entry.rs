use from_bytes::*;
use from_bytes_derive::*;
use packed_struct::prelude::*;

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct IMAGE_RESOURCE_DATA_ENTRY {
    pub OffsetToData: u32,
    pub Size: u32,
    pub CodePage: u32,
    pub Reserved: u32,
}