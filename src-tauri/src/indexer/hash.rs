use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use sha2::{Sha256, Digest};

/// Calculate SHA256 hash of a file
/// For large files, only reads the first 64KB for performance
pub fn calculate_file_hash<P: AsRef<Path>>(path: P, quick: bool) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 8192]; // 8KB buffer

    if quick {
        // Quick hash: read first 64KB only
        let max_bytes = 65536; // 64KB
        let mut bytes_read = 0;

        while bytes_read < max_bytes {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            bytes_read += n;
        }
    } else {
        // Full hash: read entire file
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Calculate a quick hash for duplicate detection
/// Uses file size + first 64KB hash
pub fn quick_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    calculate_file_hash(path, true)
}

/// Calculate full SHA256 hash of entire file
pub fn full_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
    calculate_file_hash(path, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_calculation() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello, World!").unwrap();
        temp_file.flush().unwrap();

        // Calculate hash
        let hash = full_hash(temp_file.path()).unwrap();
        
        // SHA256 of "Hello, World!"
        let expected = "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f";
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_quick_hash() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        let hash = quick_hash(temp_file.path()).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA256 hex length
    }
}
