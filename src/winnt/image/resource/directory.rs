use from_bytes::*;
use from_bytes_derive::*;
use packed_struct::prelude::*;

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct IMAGE_RESOURCE_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub NumberOfNamedEntries: u16,
    pub NumberOfIdEntries: u16,
}
