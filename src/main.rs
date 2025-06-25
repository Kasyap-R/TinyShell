mod shell;

use anyhow::Result;

fn main() -> Result<()> {
    shell::run()?;
    Ok(())
}
