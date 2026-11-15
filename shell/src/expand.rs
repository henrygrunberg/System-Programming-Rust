use std::path::Path;

pub fn expand_words(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| expand_tilde(word))
        .map(|word| expand_dot(&word))
        .flat_map(|word| expand_star(&word))
        .collect()
}

fn expand_tilde(word: &str) -> String {
    let Ok(home) = std::env::var("HOME") else {
        return word.to_string();
    };

    replace_prefix(word, '~', &home)
}

fn expand_dot(word: &str) -> String {
    let Ok(current_dir) = std::env::current_dir() else {
        return word.to_string();
    };

    replace_prefix(word, '.', &current_dir.to_string_lossy())
}

fn replace_prefix(word: &str, prefix: char, replacement: &str) -> String {
    match word.strip_prefix(prefix) {
        Some("") => replacement.to_string(),
        Some(rest) if rest.starts_with('/') => format!("{replacement}{rest}"),
        _ => word.to_string(),
    }
}

fn expand_star(word: &str) -> Vec<String> {
    if !word.contains('*') {
        return vec![word.to_string()];
    }

    let start = if word.starts_with('/') { "/" } else { "" };
    let mut paths = vec![start.to_string()];

    for component in word.split('/').filter(|part| !part.is_empty()) {
        paths = paths
            .iter()
            .flat_map(|base| expand_component(base, component))
            .collect();
    }

    if paths.is_empty() {
        return vec![word.to_string()];
    }

    paths.sort();
    paths
}

fn expand_component(base: &str, component: &str) -> Vec<String> {
    if !component.contains('*') {
        let path = Path::new(base).join(component);
        return if path.exists() {
            vec![path.to_string_lossy().into_owned()]
        } else {
            Vec::new()
        };
    }

    let directory = if base.is_empty() { "." } else { base };
    let Ok(entries) = std::fs::read_dir(directory) else {
        return Vec::new();
    };

    entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| is_visible_match(component, name))
        .map(|name| Path::new(base).join(name).to_string_lossy().into_owned())
        .collect()
}

fn is_visible_match(pattern: &str, name: &str) -> bool {
    let hidden_without_dot = name.starts_with('.') && !pattern.starts_with('.');
    !hidden_without_dot && matches_pattern(pattern, name)
}

fn matches_pattern(pattern: &str, name: &str) -> bool {
    let Some((prefix, rest)) = pattern.split_once('*') else {
        return pattern == name;
    };
    let Some(remaining) = name.strip_prefix(prefix) else {
        return false;
    };

    remaining
        .char_indices()
        .map(|(index, _)| index)
        .chain([remaining.len()])
        .any(|index| matches_pattern(rest, &remaining[index..]))
}
