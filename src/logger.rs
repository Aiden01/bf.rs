use colored::*;
use std::io;
use std::io::Write;
use std::fmt::Display;

enum Importance {
    Info,
    Success,
    Error
}

fn format_msg(importance: Importance, msg: &str) -> ColoredString {
    match importance {
        Importance::Info => msg.blue(),
        Importance::Success => msg.green(),
        Importance::Error => msg.red()
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

pub fn prompt<T: AsRef<str> + Display>(msg: T) -> String {
    let mut input = String::new();
    let log = format_msg(Importance::Info, &format!("{} ➤ ", msg));
    print!("\n{}", log);
    io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Unable to read line");
    input
}




