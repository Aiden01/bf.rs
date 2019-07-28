use colored::*;
use std::fmt::Display;
use std::io;
use std::io::Write;

enum Importance {
    Info,
    Success,
    Error,
}

fn format_msg(importance: Importance, msg: &str) -> ColoredString {
    match importance {
        Importance::Info => msg.blue(),
        Importance::Success => msg.green(),
        Importance::Error => msg.red(),
    }
}

pub fn error<T: AsRef<str>>(msg: T) {
    println!("{}", format_msg(Importance::Error, msg.as_ref()));
}

pub fn success<T: AsRef<str>>(msg: T) {
    println!("{}", format_msg(Importance::Success, msg.as_ref()));
}

pub fn info<T: AsRef<str>>(msg: T) {
    println!("{}", format_msg(Importance::Info, msg.as_ref()));
}

pub fn prompt<T: AsRef<str> + Display>(msg: T) -> Result<String, String> {
    let mut input = String::new();
    let log = format_msg(Importance::Info, &format!("{} ➤ ", msg));
    print!("\n{}", log);
    io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Cannot read input")?;
    Ok(input)
}
