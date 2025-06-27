mod builtins;
mod cmd_validation;
mod pty;

use anyhow::Result;
use builtins::{BuiltInCMD, BuiltInFn, get_builtins};
use cmd_validation::is_external_cmd;
use std::{
    collections::HashMap,
    env,
    io::{self, Write},
};

use pty::run_in_pty;

// NOTE: Later we'll store data like aliases and shell variables as shell state
pub struct TinySh {
    pub env_vars: HashMap<String, String>,
}

impl TinySh {
    fn new() -> Self {
        TinySh {
            env_vars: HashMap::new(),
        }
    }

    fn insert_env_var(&mut self, key: String, val: String) {
        self.env_vars.insert(key, val);
    }
}

pub fn run() -> Result<()> {
    let mut tiny_sh = TinySh::new();
    let mut usr_input = String::new();

    loop {
        prompt_input(&mut usr_input)?;
        let (cmd_name, args) = split_input(&usr_input);

        if cmd_name == "exit" {
            break;
        }

        if let Some(cmd_type) = BuiltInCMD::is_builtin(cmd_name) {
            run_builtin(&cmd_type, &mut tiny_sh, &args)?;
        } else if is_external_cmd(cmd_name)? {
            // For now we launch all external commands through a PTY
            run_in_pty(cmd_name, &args)?;
        } else {
            println!("Unknown Command");
        }
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

fn run_builtin(cmd_type: &BuiltInCMD, tiny_sh: &mut TinySh, args: &[&str]) -> Result<()> {
    let builtins = get_builtins();
    let handler = builtins.get(&cmd_type).unwrap();
    match handler {
        BuiltInFn::NoShellState(x) => x(&args)?,
        BuiltInFn::MutShellState(x) => x(tiny_sh, &args)?,
        BuiltInFn::ReadShellState(x) => x(&tiny_sh, &args)?,
    };
    Ok(())
}
