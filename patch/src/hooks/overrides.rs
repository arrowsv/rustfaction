use std::path::PathBuf;
use anyhow::Result;
use common::config::Config;
use crate::{hook_fn, rfg::file::{xml_parse_from_string}, utils::address::addr};

hook_fn!(
    addr(0x1CD2F0),
    extern "C" fn(file_name: *const std::ffi::c_char, dest: *mut crate::rfg::memory::MempoolBase) -> *mut crate::rfg::file::XmlElement,
    xml_parse
    {
        let file_name_str = unsafe {
            std::ffi::CStr::from_ptr(file_name)
        }.to_string_lossy();

        let override_path = get_overrides_dir().join(&*file_name_str);
        let file_string = match std::fs::read_to_string(&override_path) {
            Ok(content) => content,
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        return xml_parse_original()(file_name, dest);
                    }
                    _ => {
                        log::error!("Failed to read override file {:?}: {}", override_path, e);
                        return xml_parse_original()(file_name, dest);
                    }
                }
            }
        };

        let file_buffer = unsafe { 
            (*dest).alloc(file_string.len() as u32 + 1, 4) as *mut std::ffi::c_char 
        };

        unsafe {
            std::ptr::copy_nonoverlapping(
                file_string.as_ptr(),
                file_buffer as *mut u8,
                file_string.len()
            );

            // Null-terminate buffer
            *file_buffer.add(file_string.len()) = 0;
        }

        let root = xml_parse_from_string()(file_buffer, dest, file_name);
        if root.is_null() {
            log::error!("Failed to parse override XML file {:?}", override_path);
            return xml_parse_original()(file_name, dest);
        }

        log::info!("Loaded XML override for {}", file_name_str);
        
        root
    }
);

hook_fn!(
    addr(0x1B6790),
    extern "C" fn(
        file_path: *const std::ffi::c_char, 
        mode: *const std::ffi::c_char, 
        media_type: *mut u32, 
        async_operation: *mut u8
    ) -> *mut u32,
    keen_cf_open_file
    {
        let packfile_path_str = unsafe {
            std::ffi::CStr::from_ptr(file_path)
        }.to_string_lossy();

        let packfile_name = std::path::Path::new(&*packfile_path_str)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let override_path = get_overrides_dir().join(&*packfile_name);
        if !override_path.exists() {
            return keen_cf_open_file_original()(file_path, mode, media_type, async_operation);
        }

        let new_packfile_name_cstr = std::ffi::CString::new(override_path.to_string_lossy().as_bytes()).unwrap();
        let packfile_path = new_packfile_name_cstr.as_ptr();

        keen_cf_open_file_original()(packfile_path, mode, media_type, async_operation)
    }
);


fn get_overrides_dir() -> PathBuf {
    common::utils::get_module_dir(crate::get_dll_module())
        .expect("Failed to get module directory for overrides")
        .join("overrides")
}

pub fn apply() -> Result<()> {
    if !Config::get().use_overrides {
        return Ok(());
    }

    xml_parse_register()?;
    keen_cf_open_file_register()?;
    
    Ok(())
}