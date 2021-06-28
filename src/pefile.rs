use byteorder::{ByteOrder, LittleEndian};
use memmap::MmapOptions;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str;

use crate::winnt::IMAGE_OPTIONAL_HEADER::*;
use crate::winnt::*;
use from_bytes::StructFromBytes;
use packed_size::*;

pub trait ResourceDirectoryVisitor {
    fn init(&mut self, _pefile: &PEFile) {}
    fn finalize(&mut self, _pefile: &PEFile) {}

    fn enter_resource_directory(
        &mut self,
        pefile: &PEFile,
        dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    );
    fn leave_resource_directory(
        &mut self,
        pefile: &PEFile,
        dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    );

    fn visit_resource_data_entry(
        &mut self,
        pefile: &PEFile,
        entry: &IMAGE_RESOURCE_DATA_ENTRY,
        identifier: &EntryIdentifier,
    );
}

#[allow(dead_code)]
pub struct PEFile {
    filename: PathBuf,
    mmap: memmap::Mmap,
    image_dos_header: IMAGE_DOS_HEADER,
    image_file_header: IMAGE_FILE_HEADER,
    image_optional_header: Option<IMAGE_OPTIONAL_HEADER>,
    directories: Vec<Option<IMAGE_DATA_DIRECTORY>>,
    sections: Vec<IMAGE_SECTION_HEADER>,
}

impl PEFile {
    pub fn new(filename: PathBuf) -> std::io::Result<PEFile> {
        let mut offset = 0;
        let file = File::open(&filename)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let image_dos_header = IMAGE_DOS_HEADER::from_bytes(&mmap, offset)?;

        if image_dos_header.e_magic != LittleEndian::read_u16(b"MZ") {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("illegal DOS magic: {:?}", &mmap[0..2]),
            ));
        } else {
            log::debug!("DOS magic is ok");
        }

        let nt_magic_offset = image_dos_header.e_lfanew as usize;
        let nt_magic = &mmap[nt_magic_offset..nt_magic_offset + 4];
        if nt_magic != [b'P', b'E', 0, 0] {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("illegal NT magic: {:?}", nt_magic),
            ));
        } else {
            log::debug!("NT magic is ok");
        }
        offset = (image_dos_header.e_lfanew + 4) as usize;
        let nt_header_size = IMAGE_FILE_HEADER::packed_size();
        log::debug!(
            "searching extended header at 0x{:08x}, size = {}",
            offset,
            nt_header_size
        );
        let image_file_header = IMAGE_FILE_HEADER::from_bytes(&mmap, offset)?;
        offset += nt_header_size;

        let optional_header_size = image_file_header.SizeOfOptionalHeader as usize;
        log::debug!("size of optional header is {}", optional_header_size);

        let image_optional_header = if optional_header_size == 0 {
            None
        } else {
            let header_magic = &mmap[offset..offset + 2];
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
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("illegal optional header magic: {:?}", header_magic),
                    ));
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
                    log::debug!(
                        "DATA DIRECTORY {:02}: address = 0x{:08x}, size = {}",
                        idx,
                        entry.VirtualAddress,
                        entry.Size
                    );
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
        for idx in 0..image_file_header.NumberOfSections {
            let entry =
                IMAGE_SECTION_HEADER::from_bytes(&mmap, offset + (entry_size * idx as usize))?;

            let section_name = str::from_utf8(&entry.Name[..]).unwrap();
            let virt_size = entry.Misc;
            let virt_addr = entry.VirtualAddress;
            let raw_offset = entry.PointerToRawData;
            let raw_size = entry.SizeOfRawData;

            log::debug!(
                "{:02} {}  VirtAddr: {:08x}      VirtSize: {:08x}",
                idx,
                section_name,
                virt_addr,
                virt_size
            );
            log::debug!(
                "  raw data offs: {:08x} raw data size: {:08x} ",
                raw_offset,
                raw_size
            );

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

    pub fn get_raw_address(&self, rva: usize) -> Option<usize> {
        match self.sections.iter().find(|&x| {
            (x.VirtualAddress as usize..(x.VirtualAddress + x.Misc) as usize).contains(&rva)
        }) {
            None => None,
            Some(sect) => {
                log::debug!(
                    "found rva {:08x} in section {}",
                    rva,
                    str::from_utf8(&sect.Name[..]).unwrap()
                );
                let raw_address =
                    rva - sect.VirtualAddress as usize + sect.PointerToRawData as usize;
                log::debug!(
                    "raw address is {:08x} ({:x} - {:x} + {:x})",
                    raw_address,
                    rva,
                    sect.VirtualAddress,
                    sect.PointerToRawData
                );
                Some(raw_address)
            }
        }
    }
    
    pub fn get_resources_section(&self) -> Option<&[u8]> {
        let idx_resources =
            ToPrimitive::to_usize(&IMAGE_DIRECTORY_ENTRY::IMAGE_DIRECTORY_ENTRY_RESOURCE).unwrap();
        if let Some(entry) = &self.directories[idx_resources] {
            if let Some(offset) = self.get_raw_address(entry.VirtualAddress as usize) {
                log::debug!(
                    "loading resources of size {} at 0x{:08x}",
                    entry.Size,
                    offset
                );

                // create slice to enforce bounds checking
                return Some(&self.mmap[offset..offset + entry.Size as usize]);
            }
        }
        None
    }

    pub fn print_resources(&self) {
        let mut visitor = ConsoleVisitor::new();
        self.visit_resource_tree(&mut visitor).unwrap();
    }

    pub fn visit_resource_tree<V: ResourceDirectoryVisitor>(
        &self,
        visitor: &mut V,
    ) -> std::io::Result<()> {
        visitor.init(self);

        if let Some(resources) = self.get_resources_section() {
            self.visit_directory(resources, visitor, 0, EntryIdentifier::NoIdentifier)?;
        }

        visitor.finalize(self);
        Ok(())
    }

    pub fn visit_directory<V: ResourceDirectoryVisitor>(
        &self,
        resources: &[u8],
        visitor: &mut V,
        offset: usize,
        identifier: EntryIdentifier,
    ) -> std::io::Result<()> {
        let dir = IMAGE_RESOURCE_DIRECTORY::from_bytes(resources, offset)?;
        visitor.enter_resource_directory(self, &dir, &identifier);

        let offset = offset + IMAGE_RESOURCE_DIRECTORY::packed_size();
        let entry_size = IMAGE_RESOURCE_DIRECTORY_ENTRY::packed_size();
        let count = (dir.NumberOfNamedEntries + dir.NumberOfIdEntries) as usize;
        for idx in 0..count {
            let entry_offset = offset + idx * entry_size;
            self.visit_directory_entry(resources, visitor, entry_offset)?;
        }
        visitor.leave_resource_directory(self, &dir, &identifier);
        Ok(())
    }

    pub fn visit_directory_entry<V: ResourceDirectoryVisitor>(
        &self,
        resources: &[u8],
        visitor: &mut V,
        offset: usize,
    ) -> std::io::Result<()> {
        let raw_entry = IMAGE_RESOURCE_DIRECTORY_ENTRY::from_bytes(resources, offset)?;
        let identifier = raw_entry.parse_identifier(resources);
        if (raw_entry.OffsetToData & 0x80000000) == 0x80000000 {
            let entry_offset = raw_entry.OffsetToData & 0x7fffffff;
            self.visit_directory(resources, visitor, entry_offset as usize, identifier)?;
        } else {
            let entry_offset = raw_entry.OffsetToData as usize;
            let raw_entry = IMAGE_RESOURCE_DATA_ENTRY::from_bytes(resources, entry_offset)?;
            visitor.visit_resource_data_entry(self, &raw_entry, &identifier);
        }

        Ok(())
    }
}

pub struct ConsoleVisitor {
    level: u32,
}
impl ConsoleVisitor {
    pub fn new() -> Self {
        Self { level: 0 }
    }
    fn indent(&self) -> String {
        let mut res = String::with_capacity(self.level as usize * 2);
        for _ in 0..self.level {
            res.push_str("  ");
        }
        res
    }
    fn enter(&mut self) {
        self.level += 1;
    }
    fn leave(&mut self) {
        if self.level == 0 {
            panic!("stack underflow");
        }
        self.level -= 1;
    }
}
impl ResourceDirectoryVisitor for ConsoleVisitor {
    fn enter_resource_directory(
        &mut self,
        _pefile: &PEFile,
        _dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    ) {
        self.enter();
        println!("{}{:?}", self.indent(), identifier);
    }
    fn leave_resource_directory(
        &mut self,
        _pefile: &PEFile,
        _dir: &IMAGE_RESOURCE_DIRECTORY,
        _identifier: &EntryIdentifier,
    ) {
        self.leave();
    }

    fn visit_resource_data_entry(
        &mut self,
        _pefile: &PEFile,
        _entry: &IMAGE_RESOURCE_DATA_ENTRY,
        identifier: &EntryIdentifier,
    ) {
        println!("{} -> {:?}", self.indent(), identifier);
    }
}
