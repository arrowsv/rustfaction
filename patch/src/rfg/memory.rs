use std::ffi::{c_void, c_char};

#[repr(C)]
#[allow(dead_code)]
pub struct MempoolBase {
    vtable: *const MempoolBaseVTable,
    locked: u8, // volatile
    lock_id: u32, // volatile
    name: [c_char; 32],
    flags: u8,
    thread_id: u32, // volatile
    peak_usage: u32,
}

impl MempoolBase {
    pub fn alloc(&mut self, size: u32, alignment: u32) -> *mut c_void {
        unsafe {
            let alloc_fn = (*self.vtable).alloc.expect("MempoolBase alloc function not found");
            alloc_fn(self as *mut MempoolBase, size, alignment) 
        }
    }
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct MempoolBaseVTable {
    set_thread_ownership: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase)>,
    is_owned: Option<unsafe extern "thiscall" fn(this: *const MempoolBase) -> u8>,
    is_dynamic: Option<unsafe extern "thiscall" fn(this: *const MempoolBase) -> u8>,
    space_free: Option<unsafe extern "thiscall" fn(this: *const MempoolBase) -> u32>,
    space_used: Option<unsafe extern "thiscall" fn(this: *const MempoolBase) -> u32>,
    space_max: Option<unsafe extern "thiscall" fn(this: *const MempoolBase) -> u32>,
    can_alloc: Option<unsafe extern "thiscall" fn(this: *const MempoolBase, arg1: u32, arg2: u32) -> u8>,
    alloc: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase, arg1: u32, arg2: u32) -> *mut c_void>,
    realloc: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase, ptr: *mut c_void, size: u32) -> *mut c_void>,
    contains_address: Option<unsafe extern "thiscall" fn(this: *const MempoolBase, addr: *const c_void) -> u8>,
    clear: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase) -> u8>,
    get_base: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase) -> *mut c_void>,
    mark: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase) -> u32>,
    restore_to_mark: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase, mark: u32) -> u8>,
    release_bytes: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase, bytes: u32) -> u8>,
    pad_to_page: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase, arg1: u32) -> u8>,
    __vecDelDtor: Option<unsafe extern "thiscall" fn(this: *mut MempoolBase, arg: u32) -> *mut c_void>,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MemoryBlockDefinition {
    pub size: u32,
    alignment: u32,
    r#type: u16,
    flags: u16,
    initial_protection_mask: u32,
    description: *const c_char
}

#[repr(C)]
#[allow(dead_code)]
pub struct MemoryLayoutDefinition {
    pub block_definitions: *mut MemoryBlockDefinition,
    pub block_count: u32,
}