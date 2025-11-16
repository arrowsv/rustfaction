pub mod address;
pub mod hook;
pub mod traits;
pub mod patchers;

pub fn write_bytes(addr: usize, bytes: &[u8]) {
    use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS};
    
    unsafe { 
        let mut orig_protections = PAGE_PROTECTION_FLAGS(0);

        VirtualProtect(
            addr as *mut core::ffi::c_void, 
            bytes.len(),
            PAGE_EXECUTE_READWRITE, 
            &mut orig_protections
        );

        std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());

        VirtualProtect(
            addr as *mut core::ffi::c_void, 
            bytes.len(),
            orig_protections, 
            std::ptr::null_mut()
        );
    }
}

pub fn write_value<T: traits::ToBytes>(addr: usize, value: T) {
    let bytes = traits::ToBytes::to_le_bytes(value);
    write_bytes(addr, &bytes);
}