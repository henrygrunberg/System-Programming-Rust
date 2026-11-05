pub fn change_directory(args: &[&str]) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    let target_dir = if args.is_empty() {
        match std::env::var("HOME") {
            Ok(home) => home,
            Err(e) => {
                eprintln!("cd: HOME: {e}");
                return;
            }
        }
    } else {
        args[0].to_string()
    };

    if let Err(e) = std::env::set_current_dir(&target_dir) {
        eprintln!("cd: {target_dir}: {e}");
    }
}

pub fn list_directory(args: &[&str]) {
    if args.len() > 1 {
        eprintln!("ls: too many arguments");
        return;
    }

    let target_dir = if args.is_empty() { "." } else { args[0] };

    match read_entry_names(target_dir) {
        Ok(names) => {
            for name in names {
                println!("{name}");
            }
        }
        Err(e) => eprintln!("ls: {target_dir}: {e}"),
    }
}

fn read_entry_names(dir: &str) -> std::io::Result<Vec<String>> {
    let mut names = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if !name.starts_with('.') {
            names.push(name);
        }
    }

    names.sort_by_key(|name| name.to_lowercase());
    Ok(names)
}
