use std::path::PathBuf;
use std::fs::File;
use std::io::{Error, ErrorKind};
use memmap::MmapOptions;
use byteorder::{ByteOrder, LittleEndian};
use num_traits::FromPrimitive;

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
}

impl PEFile {
    pub fn new(filename: PathBuf) -> std::io::Result<PEFile> {
        let file=File::open(&filename)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let image_dos_header = IMAGE_DOS_HEADER::from_bytes(&mmap, 0)?;

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
        
        let nt_header = (image_dos_header.e_lfanew + 4) as usize;
        let nt_header_size = IMAGE_FILE_HEADER::packed_size();
        log::debug!("searching extended header at 0x{:08x}, size = {}", nt_header, nt_header_size);
        let image_file_header = IMAGE_FILE_HEADER::from_bytes(&mmap, nt_header)?;

        let optional_header_size = image_file_header.SizeOfOptionalHeader as usize;
        log::debug!("size of optional header is {}", optional_header_size);

        let image_optional_header = if optional_header_size == 0 {
            None
        } else {
            let header_offset = nt_header + nt_header_size;
            let header_magic = &mmap[header_offset..header_offset+2];
            match FromPrimitive::from_u16(LittleEndian::read_u16(header_magic)) {
                Some(IMAGE_NT_OPTIONAL_HEADER::IMAGE_NT_OPTIONAL_HDR32_MAGIC) => {
                    let header = IMAGE_OPTIONAL_HEADER32::from_bytes(&mmap, header_offset)?;
                    Some(x86(*header))
                }
                Some(IMAGE_NT_OPTIONAL_HEADER::IMAGE_NT_OPTIONAL_HDR64_MAGIC) => {
                    let header = IMAGE_OPTIONAL_HEADER64::from_bytes(&mmap, header_offset)?;
                    Some(AMD64(*header))
                }
                _  => {
                    return Err(Error::new(ErrorKind::InvalidData, format!("illegal optional header magic: {:?}", header_magic)));
                }
            }
        };

        let me = PEFile {
            filename,
            mmap,
            image_dos_header: *image_dos_header,
            image_file_header: *image_file_header,
            image_optional_header,
        };
        return Ok(me);
    }

    pub fn info(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Machine:  {:?}", self.image_file_header.Machine));
        if let Some(v) = &self.image_optional_header {
            lines.push(format!("Sections: {}", v.NumberOfRvaAndSizes()));
        }
        lines.join("\n")
    }
}