use std::path::PathBuf;
use std::fs::File;
use std::io::{Error, ErrorKind};
use memmap::MmapOptions;
use byteorder::{ByteOrder, LittleEndian};

use from_bytes::StructFromBytes;
use crate::winnt::IMAGE_DOS_HEADER;
use crate::winnt::IMAGE_FILE_HEADER;

#[allow(dead_code)]
pub struct PEFile {
    filename: PathBuf,
    mmap: memmap::Mmap,
    image_dos_header: IMAGE_DOS_HEADER,
    image_file_header: IMAGE_FILE_HEADER,
}

impl PEFile {
    pub fn new(filename: PathBuf) -> std::io::Result<PEFile> {
        let file=File::open(&filename)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };

        let image_dos_header = IMAGE_DOS_HEADER::from_bytes(&mmap[0..64])?;

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
        log::debug!("searching extended header at 0x{:08x}", nt_header);
        let image_file_header = IMAGE_FILE_HEADER::from_bytes(&mmap[nt_header..nt_header+20])?;

        //let image_file_header = PEFile::image_file_header(&mmap[2..22]);

        let me = PEFile {
            filename,
            mmap,
            image_dos_header: *image_dos_header,
            image_file_header: *image_file_header
        };
        return Ok(me);
    }

    pub fn info(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Machine: {:?}", self.image_file_header.Machine));
        lines.join("\n")
    }
}