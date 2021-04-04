
use packed_struct::prelude::*;
use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use packed_size::*;
use packed_size_derive::*;
use std::convert::TryInto;
use num_derive::FromPrimitive;

pub enum IMAGE_OPTIONAL_HEADER {
  AMD64(IMAGE_OPTIONAL_HEADER64),
  x86(IMAGE_OPTIONAL_HEADER32)
}

impl IMAGE_OPTIONAL_HEADER {
  pub fn NumberOfRvaAndSizes(&self) -> u32 {
    match self {
      IMAGE_OPTIONAL_HEADER::AMD64(v)  => v.NumberOfRvaAndSizes,
      IMAGE_OPTIONAL_HEADER::x86(v)    => v.NumberOfRvaAndSizes
    }
  }
}

#[derive(PrimitiveEnum_u16, PackedSize_u16, FromPrimitive, Clone, Copy, PartialEq, Debug)]
pub enum IMAGE_NT_OPTIONAL_HEADER {
    IMAGE_NT_OPTIONAL_HDR32_MAGIC = 0x10b,
    IMAGE_NT_OPTIONAL_HDR64_MAGIC = 0x20b,
    IMAGE_ROM_OPTIONAL_HDR_MAGIC  = 0x107
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_OPTIONAL_HEADER32 {
    #[packed_field(bits="0..16", ty="enum")]
    pub Magic: IMAGE_NT_OPTIONAL_HEADER,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub ImageBase: u32,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: u16,
    pub SizeOfStackReserve: u32,
    pub SizeOfStackCommit: u32,
    pub SizeOfHeapReserve: u32,
    pub SizeOfHeapCommit: u32,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
  }

  #[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
  #[packed_struct(bit_numbering="msb0", endian="lsb")]
  pub struct IMAGE_OPTIONAL_HEADER64 {
    #[packed_field(bits="0..16", ty="enum")]
    pub Magic: IMAGE_NT_OPTIONAL_HEADER,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub ImageBase: u64,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: u16,
    pub SizeOfStackReserve: u64,
    pub SizeOfStackCommit: u64,
    pub SizeOfHeapReserve: u64,
    pub SizeOfHeapCommit: u64,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
}

#[derive(FromPrimitive, Debug)]
pub enum IMAGE_DIRECTORY_ENTRY {
  IMAGE_DIRECTORY_ENTRY_EXPORT = 0,
  IMAGE_DIRECTORY_ENTRY_IMPORT = 1,
  IMAGE_DIRECTORY_ENTRY_RESOURCE = 2,
  IMAGE_DIRECTORY_ENTRY_EXCEPTION = 3,
  IMAGE_DIRECTORY_ENTRY_SECURITY = 4,
  IMAGE_DIRECTORY_ENTRY_BASERELOC = 5,
  IMAGE_DIRECTORY_ENTRY_DEBUG = 6,
  IMAGE_DIRECTORY_ENTRY_ARCHITECTURE = 7,
  IMAGE_DIRECTORY_ENTRY_GLOBALPTR = 8,
  IMAGE_DIRECTORY_ENTRY_TLS = 9,
  IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG = 10,
  IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT = 11,
  IMAGE_DIRECTORY_ENTRY_IAT = 12,
  IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT = 13,
  IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR = 14,
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(endian="lsb")]
pub struct IMAGE_DATA_DIRECTORY {
  pub VirtualAddress: u32,
  pub Size: u32,
}