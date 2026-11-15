mod builtins;
mod expand;
mod external;

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

        let raw_words: Vec<&str> = line.split_whitespace().collect();
        let words = expand::expand_words(&raw_words);

        let Some((command, args)) = words.split_first() else {
            continue;
        };

        match command.as_str() {
            "exit" => return Ok(()),
            "cd" => builtins::change_directory(args),
            "ls" => builtins::list_directory(args),
            _ => external::run(command, args),
        }
    }
}
