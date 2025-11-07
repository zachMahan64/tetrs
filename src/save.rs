use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[cfg(target_os = "windows")]
fn home_dir() -> Option<PathBuf> {
    env::var("USERPROFILE").ok().map(PathBuf::from)
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}
fn write_to_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?; // overwrites if exists
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_from_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

const CONFIG_FILENAME: &str = ".tetrs";

fn config_path() -> io::Result<PathBuf> {
    match home_dir() {
        Some(mut path) => {
            path.push(CONFIG_FILENAME);
            Ok(path)
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine home directory",
        )),
    }
}
pub fn write_config(content: &str) -> io::Result<()> {
    let path = config_path()?;
    let mut file = fs::File::create(&path)?; // truncates existing
    file.write_all(content.as_bytes())?;
    file.sync_all()?; // ensure flushed to disk
    Ok(())
}
pub fn read_config() -> io::Result<String> {
    let path = config_path()?;
    fs::read_to_string(path)
}
