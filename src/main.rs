use std::env::args;
use std::process::{Command, Stdio, exit};

#[repr(i32)]
enum ExitCode {
    MalformedArgs = 1,
    NoProcess,
    ChildSpawnFailed,
}

macro_rules! exit_with {
    ($code:expr) => {
        exit($code as i32);
    };
}

fn main() {
    let mut args = args();

    if args.next().is_none() {
        eprintln!("ERROR: CLI arguments are malformed.");
        exit_with!(ExitCode::MalformedArgs);
    }

    let base_cmd = match args.next() {
        Some(bc) => bc,
        None => {
            eprintln!("ERROR: No process provided to restart!");
            exit_with!(ExitCode::NoProcess);
        }
    };

    let status = match Command::new("pkill").arg(&base_cmd).status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Failed to spawn pkill child: {}", e);
            exit_with!(ExitCode::ChildSpawnFailed);
        }
    };

    if !status.success() {
        eprintln!("WARNING: Failed to kill process. Attempting to start new process anyways");
    }

    let cmd_args: Vec<_> = args.collect();

    match Command::new(&base_cmd)
        .args(cmd_args)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(_) => {
            println!("{} restarted successfully!", &base_cmd);
        }
        Err(e) => {
            eprintln!("ERROR: Failed to spawn {}: {}", &base_cmd, e);
            exit_with!(ExitCode::ChildSpawnFailed);
        }
    };
}
