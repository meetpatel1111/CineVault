pub mod scanner;
pub mod metadata;
pub mod hash;

pub use scanner::{MediaScanner, ScanProgress};
// MediaMetadata is used internally but not needed in public API
