use anyhow::Result;
use crate::utils::{address::addr, write_value};

pub fn apply() -> Result<()> {
    // Remove "Matchmaking" menu option
    write_value::<i8>(addr(0x12624B0), 0);

    // Remove "Spectator" menu option
    write_value::<i8>(addr(0x12624F0), 0);

    // Decrease match countdown time from 10 seconds to 5 seconds
    write_value::<i32>(addr(0x5AAD8D), 5000);

    Ok(())
}