#![windows_subsystem = "windows"]
use std::{path::PathBuf};
use anyhow::{Context, Result};
use common::{config::Config, utils::init_logging};

mod injector;
mod process;
mod app;

fn main() -> Result<()> {
    init_logging(PathBuf::from("logs"), "launcher")?;
    Config::load();

    match app::run() {
        Ok(_) => {},
        Err(e) => {
            log::error!("Launcher error: {}", e);
            common::utils::show_error_message(&format!("{}", e));
            std::process::exit(1);
        }
    }

    Ok(())
}

pub fn launch_game() -> Result<()> {
    let config = Config::get();
    let game_dir = PathBuf::from(&config.game_directory);

    if !game_dir.exists() {
        anyhow::bail!("Game directory does not exist");
    }

    let game_exe = game_dir.join("rfg.exe");
    if !game_exe.exists() {
        anyhow::bail!("Game executable not found at {}", game_exe.display());
    }

    let current_directory = std::env::current_exe()
        .context("Failed to get current executable path")?
        .parent()
        .map(|p| p.to_path_buf())
        .context("Failed to get current executable directory")?;

    let patch_dll = PathBuf::from(current_directory).join("patch.dll");
    if !patch_dll.exists() {
        anyhow::bail!("Patch DLL not found at {}", patch_dll.display());
    }

    log::info!("Starting game in suspended state");
    let process = process::Process::create_suspended(
        game_exe.to_str().context("Invalid game executable path")?
    ).context("Failed to create suspended game process")?;

    log::info!("Injecting patch");
    injector::inject_dll(
        process.get_process_handle(), 
        patch_dll.to_str().context("Invalid DLL path")?
    )?;

    log::info!("Resuming process main thread");
    process.resume()?;

    if !config.keep_launcher_open {
        log::info!("Exiting launcher");
        std::process::exit(0);
    }

    Ok(())
}