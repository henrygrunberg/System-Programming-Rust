pub fn change_directory(args: &[String]) {
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
        args[0].clone()
    };

    if let Err(e) = std::env::set_current_dir(&target_dir) {
        eprintln!("cd: {target_dir}: {e}");
    }
}

pub fn list_directory(args: &[String]) {
    if args.is_empty() {
        list_path(".", false);
        return;
    }

    let show_headers = args.len() > 1;
    for path in args {
        list_path(path, show_headers);
    }
}

fn list_path(path: &str, show_header: bool) {
    match std::fs::metadata(path) {
        Err(e) => eprintln!("ls: {path}: {e}"),
        Ok(metadata) if !metadata.is_dir() => println!("{path}"),
        Ok(_) => match read_entry_names(path) {
            Ok(names) => {
                if show_header {
                    println!("{path}:");
                }
                for name in names {
                    println!("{name}");
                }
                if show_header {
                    println!();
                }
            }
            Err(e) => eprintln!("ls: {path}: {e}"),
        },
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
