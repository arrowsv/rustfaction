use std::path::PathBuf;
use windows::{Win32::{Foundation::{HMODULE, MAX_PATH}, UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MESSAGEBOX_STYLE, MessageBoxA}}, core::PCSTR};
use anyhow::Result;

pub fn show_error_message(message: &str) {
    log::error!("{}", message);
    show_message("Error", message, MB_OK | MB_ICONERROR);
}

pub fn show_info_message(message: &str) {
    log::info!("{}", message);
    show_message("Info", message, MB_OK);
}

pub fn show_message(title: &str, message: &str, flags: MESSAGEBOX_STYLE) {
    let message_cstr = std::ffi::CString::new(message).unwrap_or_default();
    let title_cstr = std::ffi::CString::new(title).unwrap_or_default();
    
    unsafe {
        MessageBoxA(
            None,
            PCSTR(message_cstr.as_ptr() as *const u8),
            PCSTR(title_cstr.as_ptr() as *const u8),
            flags,
        );
    }
}

pub fn get_module_pathname(module: HMODULE) -> Option<std::path::PathBuf> {
    let mut buffer = vec![0u8; MAX_PATH as usize];
    
    let len = unsafe {
        windows::Win32::System::LibraryLoader::GetModuleFileNameA(
            module,
            buffer.as_mut_slice()
        ) as usize
    };

    if len == 0 || len >= buffer.len() {
        log::info!("Failed to get module filename");
        return None;
    }

    buffer.truncate(len);
    let path_str = String::from_utf8_lossy(&buffer).to_string();
    Some(std::path::PathBuf::from(path_str))
}

pub fn get_module_dir(module: HMODULE) -> Option<std::path::PathBuf> {
    get_module_pathname(module)
        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
}

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} [{}] {}",
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        record.level(),
        &record.args()
    )
}

pub fn init_logging(dir: PathBuf, name: &str) -> Result<()> {
    flexi_logger::Logger::try_with_str("info")?
        .log_to_file(
            flexi_logger::FileSpec::default()
                .directory(dir)
                .basename(name)
                .suppress_timestamp(),
        )
        .format(log_format)
        .duplicate_to_stderr(flexi_logger::Duplicate::Info)
        .start()?;

    log::info!("Rust Faction {}: {}", crate::constants::VERSION, compile_time::datetime_str!());
    Ok(())
}