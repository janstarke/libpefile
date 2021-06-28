use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use packed_size::*;
use packed_size_derive::*;
use packed_struct::prelude::*;

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct MESSAGE_RESOURCE_ENTRY {
    pub Length: u16,

    /// Indicates that the string is encoded in Unicode,
    /// if equal to the value 0x0001. Indicates that the
    /// string is encoded in ANSI, if equal to the value 0x0000.
    pub Flags: u16,
    /* pub Text: u8 */
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct MESSAGE_RESOURCE_BLOCK {
    pub LowId: u32,
    pub HighId: u32,
    pub OffsetToEntries: u32,
}


#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct MESSAGE_RESOURCE_DATA {
    pub NumberOfBlocks: u32,
    /* pub Blocks: MESSAGE_RESOURCE_BLOCK, */
}