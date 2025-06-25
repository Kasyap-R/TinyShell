use anyhow::{Context, Result, anyhow};
use std::{collections::HashMap, env, fmt::Display, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::TinySh;

#[derive(Debug, PartialEq, Hash, Eq, EnumIter)]
pub enum BuiltInCMD {
    CD,
    ECHO,
    TYPE,
    HELP,
    EXPORT,
}

impl FromStr for BuiltInCMD {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "cd" => Ok(Self::CD),
            "echo" => Ok(Self::ECHO),
            "type" => Ok(Self::TYPE),
            "help" => Ok(Self::HELP),
            "export" => Ok(Self::EXPORT),
            _ => Err(()),
        }
    }
}

impl Display for BuiltInCMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmd_string;

        match self {
            Self::CD => cmd_string = String::from("cd"),
            Self::ECHO => cmd_string = String::from("echo"),
            Self::TYPE => cmd_string = String::from("type"),
            Self::HELP => cmd_string = String::from("help"),
            Self::EXPORT => cmd_string = String::from("export"),
        }

        write!(f, "{}", cmd_string)
    }
}

pub enum BuiltInFn {
    MutShellState(fn(&mut TinySh, &[&str]) -> Result<()>),
    ReadShellState(fn(&TinySh, &[&str]) -> Result<()>),
    NoShellState(fn(&[&str]) -> Result<()>),
}

pub fn cd(args: &[&str]) -> Result<()> {
    assert_eq!(args.len(), 1);
    env::set_current_dir(args[0]).context("Failed to change directory")?;
    Ok(())
}

pub fn echo(args: &[&str]) -> Result<()> {
    for arg in args {
        print!("{} ", arg);
    }
    print!("\n");
    Ok(())
}

pub fn help(args: &[&str]) -> Result<()> {
    assert_eq!(args.len(), 0);
    for (cmd_name, _) in &get_builtins() {
        println!("{}", cmd_name);
    }
    Ok(())
}

pub fn export(tiny_sh: &mut TinySh, args: &[&str]) -> Result<()> {
    assert_eq!(args.len(), 1);
    if let Some((key, val)) = args[0].split_once("=") {
        tiny_sh.insert_env_var(key.to_string(), val.to_string());
    } else {
        return Err(anyhow!(
            "Incorrect Environment Variable formatting. Expected \"key=val\""
        ));
    }
    Ok(())
}

pub fn cmd_type(args: &[&str]) -> Result<()> {
    assert_eq!(args.len(), 1);
    if let Ok(_) = BuiltInCMD::from_str(args[0]) {
        println!("{} is a shell builtin", args[0]);
    } else {
        println!("{} is an external command", args[0]);
    }
    Ok(())
}

pub fn get_builtins() -> HashMap<BuiltInCMD, BuiltInFn> {
    // We must manually specify the hashmap contains str with static lifetimes
    let mut map = HashMap::new();
    // Ensure hashmap contains ALL enum variants
    for cmd in BuiltInCMD::iter() {
        let func = match cmd {
            BuiltInCMD::CD => BuiltInFn::NoShellState(cd),
            BuiltInCMD::ECHO => BuiltInFn::NoShellState(echo),
            BuiltInCMD::HELP => BuiltInFn::NoShellState(help),
            BuiltInCMD::EXPORT => BuiltInFn::MutShellState(export),
            BuiltInCMD::TYPE => BuiltInFn::NoShellState(cmd_type),
        };
        map.insert(cmd, func);
    }
    map
}
