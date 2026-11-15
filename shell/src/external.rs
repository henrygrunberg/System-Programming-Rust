use std::io::ErrorKind;
use std::process::Command;

pub fn run(command: &str, args: &[String]) {
    match Command::new(command).args(args).status() {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {
            eprintln!("{command}: command not found");
        }
        Err(e) => eprintln!("{command}: {e}"),
    }
}
