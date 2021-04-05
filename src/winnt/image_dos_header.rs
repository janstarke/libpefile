
use packed_struct::prelude::*;
use from_bytes::StructFromBytes;
use from_bytes_derive::StructFromBytes;
use packed_size::*;
use packed_size_derive::*;

#[derive(PackedStruct, Debug, StructFromBytes, PackedSize)]
#[packed_struct(endian="lsb")]
pub struct IMAGE_DOS_HEADER {
    pub e_magic:     u16,      /* 00: MZ Header signature */
    pub e_cblp:      u16,      /* 02: Bytes on last page of file */
    pub e_cp:        u16,      /* 04: Pages in file */
    pub e_crlc:      u16,      /* 06: Relocations */
    pub e_cparhdr:   u16,      /* 08: Size of header in paragraphs */
    pub e_minalloc:  u16,      /* 0a: Minimum extra paragraphs needed */
    pub e_maxalloc:  u16,      /* 0c: Maximum extra paragraphs needed */
    pub e_ss:        u16,      /* 0e: Initial (relative) SS value */
    pub e_sp:        u16,      /* 10: Initial SP value */
    pub e_csum:      u16,      /* 12: Checksum */
    pub e_ip:        u16,      /* 14: Initial IP value */
    pub e_cs:        u16,      /* 16: Initial (relative) CS value */
    pub e_lfarlc:    u16,      /* 18: File address of relocation table */
    pub e_ovno:      u16,      /* 1a: Overlay number */
    pub e_res:       [u16;4],  /* 1c: Reserved words */
    pub e_oemid:     u16,      /* 24: OEM identifier (for e_oeminfo) */
    pub e_oeminfo:   u16,      /* 26: OEM information; e_oemid specific */
    pub e_res2:      [u16;10], /* 28: Reserved words */
    pub e_lfanew:    u32       /* 3c: Offset to extended header */
}
