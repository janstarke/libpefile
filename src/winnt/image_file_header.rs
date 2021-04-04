
use packed_struct::prelude::*;
use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use std::convert::TryInto;

#[derive(PrimitiveEnum_u16, Clone, Copy, PartialEq, Debug)]
pub enum IMAGE_FILE_HEADER_Machine {
    IMAGE_FILE_MACHINE_I386  = 0x014c,
    IMAGE_FILE_MACHINE_IA64  = 0x0200,
    IMAGE_FILE_MACHINE_AMD64 = 0x8664
}

#[derive(PackedStruct, Debug, StructFromBytes)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_FILE_HEADER {
    #[packed_field(bits="0..16", ty="enum")]
    pub Machine:              EnumCatchAll<IMAGE_FILE_HEADER_Machine>,
    pub NumberOfSections:     u16,
    pub TimeDateStamp:        u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols:      u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics:      u16,
}