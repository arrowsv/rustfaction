use std::{sync::OnceLock, thread};
use common::{config::Config, utils::{get_module_dir, init_logging}};
use windows::
Win32::{
    Foundation::{
        BOOL, HANDLE, HMODULE
    }, System::{
        Console::{AllocConsole, FreeConsole}, 
        LibraryLoader::{DisableThreadLibraryCalls, GetModuleHandleA}, 
        SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}
    },
};
use anyhow::{Context, Result};

mod utils;
mod hooks;
mod rfg;

static DLL_MODULE: OnceLock<HMODULE> = OnceLock::new();

pub fn get_dll_module() -> HMODULE {
    *DLL_MODULE.get().expect("DLL module not set")
}

fn init_patch() -> Result<()> {
    Config::init();
    
    if Config::get().show_console {
        unsafe { AllocConsole(); }
    }

    let log_dir = get_module_dir(get_dll_module())
        .context("Failed to get patch directory for logging")?.join("logs");
    
    init_logging(log_dir, "patch")?;

    let module_base = unsafe { GetModuleHandleA(None)?.0 as usize};
    utils::address::set_module_base(module_base);

    hooks::init()?;

    Ok(())
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HANDLE,
    call_reason: u32,
    lpv_reserved: &u32,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            let module = HMODULE(dll_module.0);
            let _ = unsafe { DisableThreadLibraryCalls(module) };
            DLL_MODULE.set(module).ok();
            thread::spawn(|| {
                if let Err(e) = init_patch() {
                    common::utils::show_error_message(&format!("Error initializing patch: {}", e));
                }
            });
        }
        DLL_PROCESS_DETACH => {
            unsafe { FreeConsole(); }
        }
        _ => {}
    }
    
    BOOL(1)
}