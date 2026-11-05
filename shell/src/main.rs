mod builtins;

use std::io::Write;

fn main() -> std::io::Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();
        let bytes_read = std::io::stdin().read_line(&mut line)?;

        if bytes_read == 0 {
            println!();
            return Ok(());
        }

        let mut words = line.split_whitespace();
        let Some(command) = words.next() else {
            continue;
        };
        let args: Vec<&str> = words.collect();

        match command {
            "exit" => return Ok(()),
            "cd" => builtins::change_directory(&args),
            "ls" => builtins::list_directory(&args),
            _ => eprintln!("{command}: command not found"),
        }
    }
}
