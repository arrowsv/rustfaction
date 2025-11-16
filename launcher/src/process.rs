use std::{ffi::CString};
use windows::{core::{PCSTR, PSTR}, Win32::{Foundation::HANDLE, System::Threading::{CreateProcessA, ResumeThread, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA}}};
use anyhow::{Result};

pub struct Process(PROCESS_INFORMATION);

impl Process {
    pub fn create_suspended(
        executable_path: &str
    ) -> Result<Process> {

        let executable_path_cstr = CString::new(executable_path)?;

        let working_dir = std::path::Path::new(executable_path)
            .parent()
            .and_then(|p| p.to_str())
            .ok_or_else(|| anyhow::anyhow!("Failed to extract directory from executable path: {}", executable_path))?;
        let working_dir_cstr = CString::new(working_dir)?;

        let mut startup_info = STARTUPINFOA::default();
        let mut process_info = PROCESS_INFORMATION::default();

        startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

        let success = unsafe {
            CreateProcessA(
                PCSTR(executable_path_cstr.as_ptr() as *const u8),
                PSTR(std::ptr::null_mut()),
                None,
                None,
                false,
                CREATE_SUSPENDED,
                None,
                PCSTR(working_dir_cstr.as_ptr() as *const u8),
                &mut startup_info,
                &mut process_info
            )
        };

        if !success.as_bool() {
            anyhow::bail!("Failed to create process: {}", std::io::Error::last_os_error());
        }

        Ok(Process(process_info))
    }

    pub fn resume(&self) -> Result<()> {
        unsafe {
            ResumeThread(self.0.hThread);
        }
        Ok(())
    }

    pub fn get_process_handle(&self) -> HANDLE {
        self.0.hProcess
    }
}