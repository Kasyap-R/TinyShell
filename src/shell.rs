mod builtins;

use anyhow::{Context, Result};
use builtins::{BuiltInFn, get_builtins};
use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    process::Command,
};

// NOTE: Later we'll store data like aliases and shell variables as shell state
pub struct TinySh {
    pub env_vars: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
}

impl TinySh {
    fn new() -> Self {
        TinySh {
            env_vars: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    fn insert_env_var(&mut self, key: String, val: String) {
        self.env_vars.insert(key, val);
    }
}

pub fn run() -> Result<()> {
    let builtins = get_builtins();
    let mut tiny_sh = TinySh::new();
    let mut usr_input = String::new();
    loop {
        prompt_input(&mut usr_input)?;
        let (cmd_name, args) = split_input(&usr_input);

        if cmd_name == "exit" {
            break;
        }

        if let Some(handler) = builtins.get(cmd_name) {
            match handler {
                BuiltInFn::NoShellState(x) => x(&args)?,
                BuiltInFn::MutShellState(x) => x(&mut tiny_sh, &args)?,
                BuiltInFn::ReadShellState(x) => x(&tiny_sh, &args)?,
            };
            continue;
        }

        let result = Command::new(cmd_name)
            .args(args)
            .output()
            .context("Failed to execute user command")?;

        println!("{}", String::from_utf8_lossy(&result.stdout));
    }
    Ok(())
}

fn prompt_input(usr_input: &mut String) -> Result<()> {
    usr_input.clear();
    print!("{}> ", env::current_dir()?.display());
    io::stdout().flush()?;

    io::stdin().read_line(usr_input)?;
    usr_input.pop(); // Get rid of the newline at the end
    Ok(())
}

fn split_input(usr_input: &str) -> (&str, Vec<&str>) {
    let tokenized: Vec<&str> = usr_input.split(" ").collect();
    (tokenized[0], tokenized[1..].to_vec())
}
