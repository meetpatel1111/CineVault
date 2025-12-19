use rusqlite::{Connection, Result};
use std::path::Path;
use std::fs;

#[derive(Debug, thiserror::Error)]
pub enum BackupError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Create a backup using SQLite's VACUUM INTO
/// This works safely while the database is online.
pub fn create_backup(conn: &Connection, output_path: &str) -> Result<(), BackupError> {
    // Remove existing file if it exists, as VACUUM INTO fails if file exists
    if Path::new(output_path).exists() {
        fs::remove_file(output_path)?;
    }

    conn.execute("VACUUM INTO ?1", [output_path])?;
    Ok(())
}

/// Restore a backup by staging it for the next startup
/// Validates that the file is a valid SQLite database
pub fn restore_backup(input_path: &str, app_data_dir: &Path) -> Result<(), BackupError> {
    let input_path = Path::new(input_path);
    if !input_path.exists() {
        return Err(BackupError::Validation("Backup file not found".to_string()));
    }

    // Validate: Try to open it as SQLite
    {
        let _ = Connection::open(input_path)
            .map_err(|_| BackupError::Validation("Invalid database file".to_string()))?;
        // Could also check schema_version table
    }

    // Stage for restore
    let restore_path = app_data_dir.join("cinevault.db.restore");
    fs::copy(input_path, restore_path)?;

    Ok(())
}
