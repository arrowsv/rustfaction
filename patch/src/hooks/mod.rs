use anyhow::Result;

mod version;
mod misc;
mod multi;
mod overrides;
mod limits;

pub fn init() -> Result<()> {
    crate::utils::hook::init_minhook()?;

    limits::apply()?;
    overrides::apply()?;
    version::apply()?;
    misc::apply()?;
    multi::apply()?;
    
    Ok(())
}

