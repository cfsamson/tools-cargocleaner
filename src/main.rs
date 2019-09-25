use std::{fs, path};

fn main() {
    let current = std::env::current_dir().expect("Error reading base dir");
    walk_dirs(&current);
}

fn walk_dirs(cur_path: &path::Path) {
    let entries = fs::read_dir(cur_path).expect("Error reading current dir");
    for entry in entries {
        let entry: fs::DirEntry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        // go through all files first to check if we're in a cargo root folder
        if entry.file_name() == std::ffi::OsString::from("Cargo.toml") {
            clean(cur_path);
            return;
        }

        // if not the see if there are more folders to check
        if let Ok(meta) = entry.metadata() {
            if meta.is_dir() {
                walk_dirs(&entry.path().as_path());
            }
        }
    }
}

fn clean(absolute_path: &path::Path) {
    let canonical = absolute_path
        .canonicalize()
        .expect("Error resolving an absolute path");

    let mut cmd = std::process::Command::new("cargo")
        .current_dir(&canonical)
        .arg("clean")
        .spawn()
        .expect("Error executing command");

    cmd.wait().expect("Error in child proccess");

    println!(
        "cleaned: {}",
        canonical.to_str().unwrap_or("Invalid dir name")
    );
}
