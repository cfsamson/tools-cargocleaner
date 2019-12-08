use std::{fs, io::ErrorKind, path};

fn main() {
    match run() {
        Ok(..) => (),
        Err(e) => println!("Error: {}", e),
    };
}

fn run() -> Result<(), String> {
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
        .output();
    let output = match cmd_res {
        Ok(cmd) => cmd,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let msg = format!(r#"Couldn't find "cargo". Make sure it's available."#);
            return Err(msg);
        }
        Err(e) => return Err(format!("Error executing \"cargo\" command. {}", e)),
    };

    if !output.status.success() {
        println!("Failed: {}", absolute_path.to_str().unwrap_or("Invalid dir name"));
        println!("===== ERROR =====");
        println!("{}", String::from_utf8_lossy(&output.stderr).to_string());
        println!("=================")
    } else {
        println!(
            "cleaned: {}",
            absolute_path.to_str().unwrap_or("Invalid dir name")
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn doesnt_panic_on_invalid_toml() {
        let invalid_toml = File::create("./test_cases/invalid_toml/Cargo.toml").unwrap();
        let invalid = include_bytes!("../test_cases/invalid_toml/invalid.toml");
        if let Err(_) = doesnt_panic_on_invalid_toml_helper(invalid_toml, invalid) {
            if let Err(e) = std::fs::remove_file("./test_cases/invalid_toml/Cargo.toml") {
                panic!("PLEASE MANUALLY REMOVE `test_cases/invalid_toml/Cargo.toml`\n{}", e);
            }
        }
        
    }

    fn doesnt_panic_on_invalid_toml_helper(mut invalid_toml: File, invalid: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        invalid_toml.write_all(invalid)?;
        let path = std::path::Path::new("./test_cases/invalid_toml");
        if let Err(e) = walk_dirs(path) {
            println!("{}", e);
        }
        if let Err(e) = std::fs::remove_file("./test_cases/invalid_toml/Cargo.toml") {
            panic!("PLEASE MANUALLY REMOVE `test_cases/invalid_toml/Cargo.toml`\n{}", e);
        }
        Ok(())
    }
    
}