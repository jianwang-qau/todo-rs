use dirs::home_dir;
use std::{fs, io, path};

const RODO_DB_FILENAME: &str = ".rododb";

// Get database file path
pub fn get_db_file_path() -> path::PathBuf {
    home_dir()
        .map(|it| it.join(RODO_DB_FILENAME))
        .unwrap_or_default()
}

// Check database file exist
pub fn check_db_file() -> io::Result<()> {
    if !db_exists() {
        create_db_file()?;
    }
    Ok(())
}

// database file exist
fn db_exists() -> bool {
    let dir = get_db_file_path();
    fs::metadata(dir).is_ok()
}

// Create database file
fn create_db_file() -> io::Result<()> {
    let dir = get_db_file_path();
    fs::File::create(dir)?;
    Ok(())
}
