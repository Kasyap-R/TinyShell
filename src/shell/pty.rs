use nix::sys::termios::{SetArg, Termios, cfmakeraw, tcgetattr, tcsetattr};
use std::io::Write;
use std::{env, fs::OpenOptions};

use anyhow::Result;
use portable_pty::{CommandBuilder, MasterPty, PtyPair, PtySize, native_pty_system};

pub fn run_in_pty(cmd_name: &str, args: &[&str]) -> Result<()> {
    let (tty, orig_termios) = enter_raw_mode()?;

    // create pty
    let pair = create_pty()?;

    // build and spawn command
    let cwd = env::current_dir()?;
    let mut cmd = CommandBuilder::new(cmd_name);
    cmd.args(args);
    cmd.cwd(cwd);
    let mut child = pair.slave.spawn_command(cmd)?;

    // Drop our copy of the slave, once the child closes its copy
    // reading from the slave will return an EOF
    drop(pair.slave);

    // Create a handle to read from the slave
    print_slave_output(&pair.master)?;

    // Allow child to terminate
    let _ = child.wait()?;

    // restore terminal to cooked mode (opposite of raw)
    restore_mode(&tty, &orig_termios)?;
    Ok(())
}

// Raw mode means the shell will no longer buffer until enter, and will
// be able to forward escape sequences correctly
fn enter_raw_mode() -> Result<(std::fs::File, Termios)> {
    let tty = OpenOptions::new().read(true).write(true).open("/dev/tty")?;
    let orig = tcgetattr(&tty)?;
    let mut raw = orig.clone();
    cfmakeraw(&mut raw);
    tcsetattr(&tty, SetArg::TCSANOW, &raw)?;
    Ok((tty, orig))
}

fn restore_mode(tty: &std::fs::File, orig: &Termios) -> Result<()> {
    tcsetattr(tty, SetArg::TCSANOW, orig)?;
    Ok(())
}

fn create_pty() -> Result<PtyPair> {
    let pty_system = native_pty_system();
    Ok(pty_system.openpty(PtySize::default())?)
}

fn print_slave_output(master: &Box<dyn MasterPty + Send>) -> Result<()> {
    let mut reader = master.try_clone_reader()?;
    let mut buf: [u8; 1024] = [0; 1024];

    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        std::io::stdout().write_all(&buf[..n])?;
    }

    Ok(())
}
