
use packed_struct::prelude::*;
use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use packed_size::*;
use packed_size_derive::*;
use std::convert::TryInto;
use num_derive::FromPrimitive;

#[derive(PrimitiveEnum_u16, PackedSize_u16, FromPrimitive, Clone, Copy, PartialEq, Debug)]
pub enum IMAGE_FILE_HEADER_Machine {
    IMAGE_FILE_MACHINE_I386  = 0x014c,
    IMAGE_FILE_MACHINE_IA64  = 0x0200,
    IMAGE_FILE_MACHINE_AMD64 = 0x8664
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize, Clone, Copy)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_FILE_HEADER {
    #[packed_field(bits="0..16", ty="enum")]
    pub Machine:              IMAGE_FILE_HEADER_Machine,
    pub NumberOfSections:     u16,
    pub TimeDateStamp:        u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols:      u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics:      u16,
}