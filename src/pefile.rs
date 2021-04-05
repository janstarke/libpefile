use std::path::PathBuf;
use std::fs::File;
use std::io::{Error, ErrorKind};
use memmap::MmapOptions;
use byteorder::{ByteOrder, LittleEndian};
//use num_traits::ToPrimitive;
use num_traits::FromPrimitive;
use std::str;

use from_bytes::StructFromBytes;
use packed_size::*;
use crate::winnt::*;
use crate::winnt::IMAGE_OPTIONAL_HEADER::*;

#[allow(dead_code)]
pub struct PEFile {
    filename: PathBuf,
    mmap: memmap::Mmap,
    image_dos_header: IMAGE_DOS_HEADER,
    image_file_header: IMAGE_FILE_HEADER,
    image_optional_header: Option<IMAGE_OPTIONAL_HEADER>,
    directories: Vec<Option<IMAGE_DATA_DIRECTORY>>,
    sections: Vec<IMAGE_SECTION_HEADER>
}

impl PEFile {
    pub fn new(filename: PathBuf) -> std::io::Result<PEFile> {
        let mut offset = 0;
        let file=File::open(&filename)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let image_dos_header = IMAGE_DOS_HEADER::from_bytes(&mmap, offset)?;

        if image_dos_header.e_magic != LittleEndian::read_u16(b"MZ") {
            return Err(Error::new(ErrorKind::InvalidData, format!("illegal DOS magic: {:?}", &mmap[0..2])));
        } else {
            log::debug!("DOS magic is ok");
        }

        let nt_magic_offset = image_dos_header.e_lfanew as usize;
        let nt_magic = &mmap[nt_magic_offset .. nt_magic_offset+4];
        if nt_magic != [b'P', b'E', 0, 0] {
            return Err(Error::new(ErrorKind::InvalidData, format!("illegal NT magic: {:?}", nt_magic)));
        } else {
            log::debug!("NT magic is ok");
        }
        
        offset = (image_dos_header.e_lfanew + 4) as usize;
        let nt_header_size = IMAGE_FILE_HEADER::packed_size();
        log::debug!("searching extended header at 0x{:08x}, size = {}", offset, nt_header_size);
        let image_file_header = IMAGE_FILE_HEADER::from_bytes(&mmap, offset)?;
        offset += nt_header_size;

        let optional_header_size = image_file_header.SizeOfOptionalHeader as usize;
        log::debug!("size of optional header is {}", optional_header_size);

        let image_optional_header = if optional_header_size == 0 {
            None
        } else {
            let header_magic = &mmap[offset..offset+2];
            match FromPrimitive::from_u16(LittleEndian::read_u16(header_magic)) {
                Some(IMAGE_NT_OPTIONAL_HEADER::IMAGE_NT_OPTIONAL_HDR32_MAGIC) => {
                    let header = IMAGE_OPTIONAL_HEADER32::from_bytes(&mmap, offset)?;
                    offset += IMAGE_OPTIONAL_HEADER32::packed_size();
                    Some(x86(*header))
                }
                Some(IMAGE_NT_OPTIONAL_HEADER::IMAGE_NT_OPTIONAL_HDR64_MAGIC) => {
                    let header = IMAGE_OPTIONAL_HEADER64::from_bytes(&mmap, offset)?;
                    offset += IMAGE_OPTIONAL_HEADER64::packed_size();
                    Some(AMD64(*header))
                }
                _  => {
                    return Err(Error::new(ErrorKind::InvalidData, format!("illegal optional header magic: {:?}", header_magic)));
                }
            }
        };

        log::debug!("offset is at {:08x}", offset);

        // load data directory
        let mut directories = Vec::new();
        if let Some(oh) = &image_optional_header {
            let entry_count = oh.NumberOfRvaAndSizes() as usize;
            let entry_size = IMAGE_DATA_DIRECTORY::packed_size();
            for idx in 0..entry_count {
                let entry = IMAGE_DATA_DIRECTORY::from_bytes(&mmap, offset + (entry_size * idx))?;

                if entry.VirtualAddress != 0 {
                    log::debug!("DATA DIRECTORY {:02}: address = 0x{:08x}, size = {}", idx, entry.VirtualAddress, entry.Size);
                    directories.push(Some(*entry));
                } else {
                    log::debug!("DATA DIRECTORY {:02}: <EMPTY>", idx);
                    directories.push(None);
                }
            }

            offset += entry_size * entry_count;
        }

        // load section headers
        let mut sections = Vec::new();
        let entry_size = IMAGE_SECTION_HEADER::packed_size();
        for idx in 0 .. image_file_header.NumberOfSections {
            let entry = IMAGE_SECTION_HEADER::from_bytes(&mmap, offset + (entry_size * idx as usize))?;

            let section_name = str::from_utf8(&entry.Name[..]).unwrap();
            let virt_size  = entry.Misc;
            let virt_addr  = entry.VirtualAddress;
            let raw_offset = entry.PointerToRawData;
            let raw_size   = entry.SizeOfRawData;

            log::debug!("{:02} {}  VirtAddr: {:08x}      VirtSize: {:08x}", idx, section_name, virt_addr, virt_size);
            log::debug!("  raw data offs: {:08x} raw data size: {:08x} ",raw_offset, raw_size);

            sections.push(*entry);
        }

        let me = PEFile {
            filename,
            mmap,
            image_dos_header: *image_dos_header,
            image_file_header: *image_file_header,
            image_optional_header,
            directories,
            sections,
        };
        return Ok(me);
    }
    
    #[allow(dead_code)]
    pub fn info(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Machine:  {:?}", self.image_file_header.Machine));
        if let Some(v) = &self.image_optional_header {
            lines.push(format!("Sections: {}", v.NumberOfRvaAndSizes()));
        }
        lines.join("\n")
    }

    pub fn list_resources(&self) {
        /*
        let idx_resources = ToPrimitive::to_usize(&IMAGE_DIRECTORY_ENTRY::IMAGE_DIRECTORY_ENTRY_RESOURCE).unwrap();
        if let Some(entry) = &sections[idx_resources] {
            // create slice to enforce bounds checking
            let rva = entry.VirtualAddress as usize;
            log::debug!("loading resources of size {} at 0x{:08x}", entry.Size, rva);
            //let resources = &mmap[rva .. rva+entry.Size as usize];
            let entry_size = IMAGE_RESOURCE_DIRECTORY_ENTRY::packed_size();

            let root = IMAGE_RESOURCE_DIRECTORY::from_bytes(&mmap, rva)?;
            let offset = rva + IMAGE_RESOURCE_DIRECTORY::packed_size();

            for idx in 0..root.NumberOfIdEntries {
                let e = IMAGE_RESOURCE_DIRECTORY_ENTRY::from_bytes(&mmap, offset + (entry_size * idx as usize))?;
                log::debug!("Name = {}, Offset = 0x{:08x}", e.Name, e.OffsetToData);
            }
        }
*/
        ()
    }
}