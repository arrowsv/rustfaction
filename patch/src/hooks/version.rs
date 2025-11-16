use std::{ffi::{CString, c_char}, sync::OnceLock};
use anyhow::Result;
use crate::{hook_fn, utils::address::addr};

static VERSION_CSTR: OnceLock<CString> = OnceLock::new();

hook_fn!(
    addr(0x58740),
    extern "C" fn() -> *const c_char,
    keen_get_build_version_string {
        VERSION_CSTR.get_or_init(|| {
            CString::new(
                format!("Rust Faction {}", common::constants::VERSION)
            ).expect("Failed to create CString")
        }).as_ptr()
    }
);

pub fn apply() -> Result<()> {
    keen_get_build_version_string_register()?;
    
    Ok(())
}