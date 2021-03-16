use ansi_term::Colour::*;
use std::io::Write;
use std::{
    ffi::OsString,
    io::{BufRead, BufReader},
};

/// 1. Read file and include_bytes!()
/// 2. For each line (12) add some info (from an array)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_bytes!("arch_logo.txt").to_vec();
    let file = file.as_slice();
    let contents = BufReader::new(file).lines();
    let info = os_info::get();
    let mem = sys_info::mem_info()?;
    let mut stdout = std::io::stdout();

    for (i, line) in contents.enumerate() {
        let line = line.unwrap_or("".to_string());
        let line = Blue.paint(line);
        write!(stdout, "{}", line)?;
        match i {
            0 => {
                writeln!(stdout, "OS: {} ({})", info.os_type(), std::env::consts::OS)?;
            }
            1 => {
                writeln!(stdout, "Arch: {}", std::env::consts::ARCH)?;
            }
            2 => {
                writeln!(stdout, "Edition: {} {}", info.version(), info.bitness())?;
            }
            3 => {
                writeln!(stdout, "Shell: {}", os_string("SHELL"))?;
            }
            4 => {
                writeln!(stdout, "Editor: {}", os_string("EDITOR"))?;
            }
            5 => {
                writeln!(
                    stdout,
                    "Ram: {}MB/{}MB",
                    (mem.total - mem.avail) / 1024,
                    mem.total / 1024
                )?;
            }
            10 => {
                for i in (0..7).into_iter() {
                    write!(stdout, "{}", Fixed(i).paint("███"))?;
                }
                write!(stdout, "\n")?;
            }
            11 => {
                for i in (8..15).into_iter() {
                    write!(stdout, "{}", Fixed(i).paint("███"))?;
                }
                write!(stdout, "\n")?;
            }
            _ => {
                write!(stdout, "\n")?;
            }
        }
    }
    let _ = stdout.flush();
    Ok(())
}
/// Gets an env var
fn os_string(string: &str) -> String {
    std::env::var_os(string)
        .unwrap_or(OsString::from("unable to determine."))
        .into_string()
        .unwrap()
}
