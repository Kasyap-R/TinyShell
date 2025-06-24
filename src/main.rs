mod shell;

use anyhow::Result;
use shell::TinySh;

fn main() -> Result<()> {
    TinySh::run()?;
    Ok(())
}
