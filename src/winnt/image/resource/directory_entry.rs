use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use packed_size::*;
use packed_size_derive::*;
use packed_struct::prelude::*;
use crate::utils::*;
/*
use crate::pefile::*;
use crate::winnt::*;
*/

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering = "msb0", endian = "lsb")]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    pub Name: u32,
    pub OffsetToData: u32,
}

impl IMAGE_RESOURCE_DIRECTORY_ENTRY {
    pub fn parse_identifier(&self, resources: &[u8]) -> EntryIdentifier {
        if self.is_named_entry() {
            let offset_to_name = (self.Name & 0x7fffffff) as usize;
            let Length = (resources[offset_to_name] as u16 | ((resources[offset_to_name+1] as u16)<<8 as u8)) as usize;
            let Name = utf16_from_slice(resources, offset_to_name+2, Length);
            EntryIdentifier::Name(Name.to_string())
        } else {
            EntryIdentifier::Id((self.Name & 0x0000ffff) as u16)
        }
    }

    /// Directory entries can be named or identified by an ID value.
    /// This is consistent with resources in an .RC file where you can specify
    /// a name or an ID for a resource instance. In the directory entry, when
    /// the high bit of the first DWORD is set, the remaining 31 bits are an
    /// offset to the string name of the resource. If the high bit is clear,
    /// the bottom 16 bits contain the ordinal identifier.
    /// 
    /// (https://docs.microsoft.com/en-us/archive/msdn-magazine/2002/march/inside-windows-an-in-depth-look-into-the-win32-portable-executable-file-format-part-2)
    fn is_named_entry(&self) -> bool {
        (self.Name & 0x80000000) == 0x80000000
    }
}

#[derive(Debug, Clone)]
pub enum EntryIdentifier {
    Name(String),
    Id(u16),
    NoIdentifier,
}