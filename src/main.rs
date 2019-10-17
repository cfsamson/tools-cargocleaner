use std::{fs, io::ErrorKind, path};

fn main() -> Result<(), String> {
    let current =
        std::env::current_dir().map_err(|_| format!("Error reading the base direactory"))?;
    walk_dirs(&current)?;
    Ok(())
}

fn walk_dirs(cur_path: &path::Path) -> Result<(), String> {
    // go through all files first to check if we're in a cargo root folder
    let entries = fs::read_dir(cur_path)
        .map_err(|_| format!("Error reading dir: {}", cur_path.to_string_lossy()))?;

    for entry in entries {
        let entry: fs::DirEntry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if entry.file_name() == std::ffi::OsString::from("Cargo.toml") {
            return clean(cur_path);
        }
    }

    // if not the see if there are more folders to check
    let entries = fs::read_dir(cur_path)
        .map_err(|_| format!("Error reading dir: {}", cur_path.to_string_lossy()))?;

    for entry in entries {
        let entry: fs::DirEntry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if let Ok(meta) = entry.metadata() {
            if meta.is_dir() {
                walk_dirs(&entry.path().as_path())?;
            }
        }
    }
    Ok(())
}

fn clean(absolute_path: &path::Path) -> Result<(), String> {
        let canonical = absolute_path
        .canonicalize()
        .expect("Error resolving an absolute path");

    let cmd_res = std::process::Command::new("cargo")
        .current_dir(&canonical)
        .arg("clean")
        .spawn();
    let mut cmd = match cmd_res {
        Ok(cmd) => cmd,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let msg = format!(r#"Couldn't find "cargo". Make sure it's available."#);
            return Err(msg);
        }
        Err(e) => return Err(format!("Error executing \"cargo\" command. {}", e)),
    };

    cmd.wait().expect("Error in child proccess");

    println!(
        "cleaned: {}",
        absolute_path.to_str().unwrap_or("Invalid dir name")
    );

    Ok(())
}
