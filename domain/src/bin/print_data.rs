use std::ops::Deref;

use domain::data::MODS;

fn main() -> anyhow::Result<()> {
    serde_json::to_writer_pretty(std::io::stdout(), MODS.deref())?;
    Ok(())
}
