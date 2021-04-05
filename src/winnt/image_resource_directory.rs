
use packed_struct::prelude::*;
use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use packed_size::*;
use packed_size_derive::*;

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_RESOURCE_DIRECTORY
{
    pub Characteristics: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub NumberOfIdEntries: u16,
    pub NumberOfNamedEntries: u16,
    pub TimeDateStamp: u32, 
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_RESOURCE_DATA_ENTRY
{
    pub CodePage: u32,
    pub OffsetToData: u32,
    pub Reserved: u32,
    pub Size: u32,
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY
{
    pub Name: u32,
    pub OffsetToData: u32,
}