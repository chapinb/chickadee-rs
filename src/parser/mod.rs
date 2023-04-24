pub mod compressed;
pub mod plain;

use anyhow::Result;
use std::io::{Read, Seek};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum SourceFileType {
    NotAFile,
    Plain,
    Gzip,
}

pub fn determine_file_type(file_path: &Path) -> Result<SourceFileType> {
    if !file_path.exists() || file_path.is_dir() {
        return Ok(SourceFileType::NotAFile);
    }

    let mut file = std::fs::File::open(file_path)?;
    // In the future we may need to extend this in order to handle
    // new file formats that we have support for.
    let mut header = [0u8; 3];
    file.read_exact(&mut header)?;
    file.seek(std::io::SeekFrom::Start(0))?;

    if header == [0x1f, 0x8b, 0x08] {
        // If the header matches the magic number, it's a gzip file
        // Source: https://www.garykessler.net/library/file_sigs.html
        Ok(SourceFileType::Gzip)
    } else {
        // Otherwise, we will treat it like a plain text file
        Ok(SourceFileType::Plain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_determine_file_type_str() {
        let actual = determine_file_type(Path::new("not-a-file.txt")).unwrap();
        assert_eq!(SourceFileType::NotAFile, actual);
    }

    #[test]
    fn test_determine_file_type_dir() {
        let source = tempfile::TempDir::new().unwrap();
        let actual = determine_file_type(source.path()).unwrap();
        assert_eq!(SourceFileType::NotAFile, actual);
    }

    #[test]
    fn test_determine_file_type_plain() {
        // Setup plain text file
        let mut plain_text_file = NamedTempFile::new().unwrap();
        plain_text_file.write_all(b"1.1.1.1\n2.2.2.2").unwrap();

        let actual = determine_file_type(Path::new(plain_text_file.path()));
        assert_eq!(SourceFileType::Plain, actual.unwrap());
    }

    #[test]
    fn test_determine_file_type_gzip() {
        // Setup gzip file
        let mut gzip_file = NamedTempFile::new().unwrap();
        flate2::write::GzEncoder::new(gzip_file.as_file_mut(), flate2::Compression::default())
            .write_all(b"1.1.1.1\n2.2.2.2")
            .unwrap();

        let actual = determine_file_type(Path::new(gzip_file.path()));
        assert_eq!(SourceFileType::Gzip, actual.unwrap());
    }
}
