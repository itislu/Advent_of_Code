use std::env;
use std::fs;
use std::io;
use std::path;

pub fn read_file(filename: &str) -> String {
    let dir = current_mod_dir().expect("Failed to get directory of current module");
    let path = dir.join(filename);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read file {}", path.display()))
}

fn current_mod_dir() -> io::Result<path::PathBuf> {
    let cwd = env::current_dir()?;
    let exe = env::current_exe()?;
    let module = exe
        .file_name()
        .and_then(|f| f.to_str())
        .map(|s| {
            s.chars()
                .take_while(|c| c.is_alphanumeric())
                .collect::<String>()
        })
        .unwrap_or_default();
    let mut res = cwd.clone();

    loop {
        if let Some(component) = res.file_name() {
            if exe
                .components()
                .rev()
                .skip(1)
                .any(|d| d.as_os_str() == component)
            {
                break;
            }
        }
        if !res.pop() {
            res = cwd;
            break;
        }
    }
    res.push(module);
    Ok(res)
}

pub fn debug_paths() {
    // 1. CARGO_MANIFEST_DIR
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        eprintln!("\n=== CARGO_MANIFEST_DIR ===");
        eprintln!("{}", manifest_dir);
    }

    // 2. PWD
    if let Ok(pwd) = env::var("PWD") {
        eprintln!("\n=== PWD ===");
        eprintln!("{}", pwd);
    }

    // 3. Current working directory
    if let Ok(cwd) = env::current_dir() {
        eprintln!("\n=== current_dir() ===");
        eprintln!("{:?}", cwd);
    }

    // 4. Executable path
    if let Ok(exe) = env::current_exe() {
        eprintln!("\n=== current_exe() ===");
        eprintln!("{:?}", exe);
    }

    // 5. Source file location
    eprintln!("\n=== file!() ===");
    eprintln!("{}", file!());

    // 6. Source absolute path
    if let Ok(canonical) = fs::canonicalize(file!()) {
        eprintln!("\n=== canonicalize(file!()) ===");
        eprintln!("{:?}", canonical);
    }

    // 7. Home directory
    if let Ok(home) = env::var("HOME") {
        eprintln!("\n=== HOME ===");
        eprintln!("{}", home);
    }

    // 8. OUT_DIR (only available during build)
    if let Ok(out_dir) = env::var("OUT_DIR") {
        eprintln!("\n=== OUT_DIR ===");
        eprintln!("{}", out_dir);
    }

    // 9. Current module directory (custom)
    if let Ok(mod_dir) = current_mod_dir() {
        eprintln!("\n=== current_mod_dir() ===");
        eprintln!("{:?}", mod_dir);
    }

    eprintln!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_read_file() {
        let result = read_file("Cargo.toml");
        println!("{}", result);
    }

    #[test]
    fn print_debug_paths() {
        debug_paths();
    }
}
