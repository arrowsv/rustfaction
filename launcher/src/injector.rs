use std::{ffi::{c_void, CString}, path::Path};
use anyhow::{Context, Result};

use windows::{Win32::{Foundation::{GetLastError, HANDLE}, System::{Diagnostics::Debug::WriteProcessMemory, LibraryLoader::{GetModuleHandleA, GetProcAddress}, Memory::{MEM_COMMIT, PAGE_EXECUTE_READWRITE, VirtualAllocEx}, Threading::{CreateRemoteThread}}}, core::PCSTR};


pub fn inject_dll(process_handle: HANDLE, dll_path: &str) -> Result<()> {
    log::debug!("Attempting to inject DLL: {}", dll_path);
    
    if !Path::new(dll_path).exists() {
        anyhow::bail!("DLL path does not exist: {}", dll_path);
    }

    let dll_path_cstr = CString::new(dll_path)?;
    let dll_path_size = dll_path_cstr.as_bytes_with_nul().len();
    
    log::debug!("DLL path size: {}", dll_path_size);

    unsafe {
        let remote_memory = VirtualAllocEx(
            process_handle,
            None,
            dll_path_size,
            MEM_COMMIT,
            PAGE_EXECUTE_READWRITE
        );

        if remote_memory.is_null() {
            anyhow::bail!("Failed to allocate memory in target process");
        }
        log::debug!("Remote memory allocated at: {:?}", remote_memory);

        let write_result = WriteProcessMemory(
                process_handle,
                remote_memory,
                dll_path_cstr.as_bytes_with_nul().as_ptr() as *const c_void,
                dll_path_size,
                None
            );

        if write_result.0 == 0 {
            let error = GetLastError();
            anyhow::bail!("Failed to write DLL path. Error: {}", error.0);
        }

        let kernel32_handle = GetModuleHandleA(PCSTR(b"kernel32.dll\0".as_ptr()))
            .context("Failed to get kernel32 handle")?;

        let load_library_addr = GetProcAddress(kernel32_handle, PCSTR(b"LoadLibraryA\0".as_ptr()))
            .context("Failed to get LoadLibraryA address")?;

        log::debug!("LoadLibraryA address: {:?}", load_library_addr);

        if load_library_addr as usize == 0 {
            anyhow::bail!("LoadLibraryA address is 0");
        }

        log::debug!("Creating remote thread...");
        let remote_thread = CreateRemoteThread(
                process_handle,
                None,
                0,
                Some(std::mem::transmute(load_library_addr as usize)),
                Some(remote_memory),
                0,
                None,
            ).context("Failed to create remote thread")?;

        log::debug!("Remote thread created: {:?}", remote_thread);
    }

    Ok(())
}