use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Main database connection manager
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Create a new database connection
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Enable WAL mode for better concurrency
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        
        Ok(Database {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Get a reference to the connection
    pub fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }

    /// Run database migrations
    pub fn migrate(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        super::migrations::run_migrations(&conn)
    }
}
