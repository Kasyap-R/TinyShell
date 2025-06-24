use anyhow::{Context, Result};
use std::{
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

// NOTE: Later we'll store data like aliases and shell variables as shell state
pub struct TinySh {}

impl TinySh {
    pub fn run() -> Result<()> {
        let mut usr_input = String::new();
        while usr_input != "quit" {
            usr_input.clear();
            print!("tinysh> ");
            io::stdout().flush()?;

            io::stdin().read_line(&mut usr_input)?;
            usr_input.pop(); // Get rid of the newline at the end

            let tokenized: Vec<&str> = usr_input.split(" ").collect();
            let cmd_name = tokenized[0];

            let result = Command::new(cmd_name)
                .args(&tokenized[1..])
                .output()
                .context("Failed to execute user command")?;

            println!("{}", String::from_utf8_lossy(&result.stdout));
        }
        Ok(())
    }
}
