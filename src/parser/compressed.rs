use std::path::Path;
use std::io::{Read, Write};
use anyhow::Result;
use flate2;

fn gunzip(file_path: &Path) -> Result<Vec<u8>> {
    let mut file = flate2::read::GzDecoder::new(std::fs::File::open(file_path)?);
    let mut buffer = Vec::with_capacity(8192);
    file.read_to_end(&mut buffer).unwrap();
    Ok(buffer)
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use flate2::Compression;

    #[test]
    fn test_gunzip() {
        // Setup test data
        let file_path = NamedTempFile::new().unwrap();
        let mut file = flate2::write::GzEncoder::new(std::fs::File::create(file_path.path()).unwrap(), Compression::default());
        file.write_all(b"1.1.1.1\n2.2.2.2").unwrap();
        file.finish().unwrap();

        // Run test
        let buffer = gunzip(file_path.path());
        assert!(buffer.is_ok());
        let buffer = buffer.unwrap();
        assert_eq!(b"1.1.1.1\n2.2.2.2", &buffer[..]);
    }
}