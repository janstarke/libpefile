
use packed_struct::prelude::*;
use from_bytes::*;
use from_bytes_derive::*;

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(endian="lsb")]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8;8],
    pub Misc: u32, /* PhysicalAddress or VirtualSize */
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: u32,
  }