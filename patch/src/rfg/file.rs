use std::ffi::c_char;
use crate::{addr_fn, rfg::memory::MempoolBase, utils::address::addr};

#[repr(C)]
#[allow(dead_code)]
pub struct XmlElement {
    name: *const std::ffi::c_char,
    next: *mut XmlElement,
    elements: *mut XmlElement,
    text: *mut std::ffi::c_char,
}

#[repr(C)]
#[allow(dead_code)]
pub struct CFile {}

#[repr(u32)]
#[allow(dead_code)]
pub enum CFileSearchType {
    None = 0xFFFF,
    Standard = 0,
    PackFile = 1,
    VDir = 2,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum VLibPlatform {
    None = 0xFFFF,
    Pc = 0,
    Ps2 = 1,
    Ps3 = 2,
    Xbox = 3,
    Xbox2 = 4,
    Xbox1 = 5,
    Ps4 = 6,
    Switch = 7,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum CFileIOMediaType {
    None = 0xFFFF,
    Hdd = 0,
    Dvd = 1,
    Host = 2,
    Memory = 3,
}

#[repr(C)]
#[allow(dead_code)]
pub struct CfFoundInfo {
    search_type: CFileSearchType,
    media_type: CFileIOMediaType,
    pub file_name: [c_char; 0x100],
    pub size: u32
}


addr_fn!(
    addr(0x1BF870),
    extern "C" fn(
        buffer: *mut c_char, 
        dest: *mut MempoolBase, 
        file_name_orig: *const c_char
    ) -> *mut XmlElement,
    xml_parse_from_string
);

addr_fn!(
    addr(0x1CADA0),
    extern "C" fn(
        packfile_name: *const c_char, 
        preload: *mut MempoolBase, 
        installed_packfile: u8, 
        encrypted: u8
    ) -> u8,
    packfile_add
);