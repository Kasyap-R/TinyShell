use anyhow::{Context, Result, anyhow};
use std::{collections::HashMap, env};

use super::TinySh;

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

pub fn get_builtins() -> HashMap<&'static str, BuiltInFn> {
    // We must manually specify the hashmap contains str with static lifetimes
    let mut map: HashMap<&'static str, BuiltInFn> = HashMap::new();
    map.insert("cd", BuiltInFn::NoShellState(cd));
    map.insert("echo", BuiltInFn::NoShellState(echo));
    map.insert("help", BuiltInFn::NoShellState(help));
    map.insert("export", BuiltInFn::MutShellState(export));
    map
}
