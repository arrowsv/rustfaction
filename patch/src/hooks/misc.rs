use anyhow::Result;
use common::config::Config;
use crate::hook_fn;
use crate::rfg::game::{exit_startup_videos, GameState};
use crate::utils::{address::addr, write_value};

hook_fn!(
    addr(0x3D87E0),
    extern "C" fn(new_state: GameState, transparent: u8, pause_beneath: u8) -> (),
    gameseq_push_state {
        if new_state == GameState::MultiplayerSearchMatchmaking 
        || new_state == GameState::MultiplayerChangeMatchmaking {
            return;
        }
        gameseq_push_state_original()(new_state, transparent, pause_beneath);
    }
);

hook_fn!(
    addr(0x1D2420),
    extern "C" fn() -> (),
    rfg_do_frame
    {
        rfg_do_frame_original()()
    }
);

hook_fn!(
    addr(0x18AD30),
    extern "C" fn(min: f32, max: f32) -> (),
    frametime_set_cap
    {
        let fps = 1.0 / Config::get().fps_limit as f32;
        frametime_set_cap_original()(fps, max)
    }
);

hook_fn!(
    addr(0x1D2360),
    extern "C" fn() -> u8,
    rfg_init_stage_1_loop_update
    {
        if Config::get().fast_start {
            unsafe { *exit_startup_videos() = true; }
        }
        rfg_init_stage_1_loop_update_original()()
    }
);

hook_fn!(
    addr(0x1AF260),
    extern "C" fn(text: *const std::ffi::c_char, color: u32, chat_text: u8) -> (),
    console_update
    {
        let text_str = unsafe { std::ffi::CStr::from_ptr(text).to_str().unwrap_or("Invalid text")};
        let text_str = text_str.replace("\n", "");
        log::info!("{}", text_str);
    }
);

pub fn apply() -> Result<()> {
    gameseq_push_state_register()?;
    rfg_init_stage_1_loop_update_register()?;
    rfg_do_frame_register()?;
    frametime_set_cap_register()?;
    console_update_register()?;

    // Enables saving while cheats are active.
    // void stats_set_cheat(cheat_indexes cheat_index, uint8_t enable)
    write_value::<i8>(addr(0x3EADED), 0);

    Ok(())
}