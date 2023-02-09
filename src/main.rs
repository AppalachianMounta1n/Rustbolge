use std::process::Command;
use std::io;

fn main() {
    let mut input = String::new();
    io:stdin().read_line(&mut input).expect("Failed to read input.");

    let mut cmd: [String; 2] = ["", ""];

    for i in input {
        
    }

    let output = Command::new("echo").arg(cmd).output().expect();
}