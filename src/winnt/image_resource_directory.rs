
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
    pub TimeDateStamp: u32, 
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub NumberOfNamedEntries: u16,
    pub NumberOfIdEntries: u16,
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_RESOURCE_DATA_ENTRY
{
    pub OffsetToData: u32,
    pub Size: u32,
    pub CodePage: u32,
    pub Reserved: u32,
}

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY
{
    pub Name: u32,
    pub OffsetToData: u32,
}

pub enum DirectoryEntryType {
    Directory(ImageResourceDirectory),
    Data(ImageResourceDataEntry)
}

pub struct ImageResourceDirectory {
    pub raw_directory: IMAGE_RESOURCE_DIRECTORY,
    pub children: Vec<DirectoryEntryType>,
}

impl ImageResourceDirectory {
    pub fn from_bytes(resources: &[u8], offset: usize) -> std::io::Result<ImageResourceDirectory> {
        log::trace!("ImageResourceDirectory::from_bytes(..., offset = {:08x})", offset);
        let raw_directory = IMAGE_RESOURCE_DIRECTORY::from_bytes(&resources, offset)?;
    
        let children = Self::parse_directory_entry_list(
            resources,
            offset + IMAGE_RESOURCE_DIRECTORY::packed_size(),
            (raw_directory.NumberOfNamedEntries + raw_directory.NumberOfIdEntries) as usize)?;

        Ok(ImageResourceDirectory {
            raw_directory: *raw_directory,
            children
        })
    }

    fn parse_directory_entry_list(resources: &[u8], offset: usize, count: usize) -> std::io::Result<Vec<DirectoryEntryType>> {
        log::trace!("parse_directory_entry_list(..., offset={:08x}, count={:08x})", offset, count);
        let entry_size = IMAGE_RESOURCE_DIRECTORY_ENTRY::packed_size();
        let mut children = Vec::new();
        for idx in 0..count {
            let entry_offset = offset + idx * entry_size;
            let e = IMAGE_RESOURCE_DIRECTORY_ENTRY::from_bytes(&resources, entry_offset)?;
            log::debug!("directory entry {:0}: Name = {}, Offset = 0x{:08x}", idx+1, e.Name, e.OffsetToData);
            let child = Self::parse_directory_entry(&resources, &e)?;
            children.push(child);
        }
        Ok(children)
    }

    fn parse_directory_entry(resources: &[u8], e: &IMAGE_RESOURCE_DIRECTORY_ENTRY) -> std::io::Result<DirectoryEntryType> {
        log::trace!("parse_directory_entry(..., OffsetToData={:08x})", e.OffsetToData);
        if (e.OffsetToData & 0x80000000) == 0x80000000 {
            log::trace!("found directory entry");
            let entry_offset = e.OffsetToData & 0x7fffffff;
            let child = ImageResourceDirectory::from_bytes(&resources, entry_offset as usize)?;
            Ok(DirectoryEntryType::Directory(child))
        } else {
            log::trace!("found data entry");
            let entry_offset = e.OffsetToData;
            let child = ImageResourceDataEntry::from_bytes(&resources, entry_offset as usize)?;
            Ok(DirectoryEntryType::Data(child))
        }
    }
}

pub struct ImageResourceDataEntry {
    pub raw_entry: IMAGE_RESOURCE_DATA_ENTRY,
}

impl ImageResourceDataEntry {
    pub fn from_bytes(resources: &[u8], offset: usize) -> std::io::Result<ImageResourceDataEntry> {
        log::trace!("ImageResourceDataEntry::from_bytes(..., offset = {:08x})", offset);
        let raw_entry = IMAGE_RESOURCE_DATA_ENTRY::from_bytes(&resources, offset)?;
        Ok(ImageResourceDataEntry {
            raw_entry: *raw_entry
        })
    }
}