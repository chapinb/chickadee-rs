pub mod compressed;
pub mod plain;

use anyhow::Result;
use flate2::read::GzDecoder;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum FileType {
    NotAFile,
    Plain,
    Gzip,
}

fn determine_file_type(file_path: &Path) -> Result<FileType> {
    if !file_path.exists() {
        return Ok(FileType::NotAFile);
    }

    let file = std::fs::File::open(file_path).unwrap();

    if flate2::read::GzDecoder::new(file).header().is_some() {
        Ok(FileType::Gzip)
    } else {
        Ok(FileType::Plain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_determine_file_type() {
        // Setup plain text file
        let mut plain_text_file = NamedTempFile::new().unwrap();
        plain_text_file.write_all(b"1.1.1.1\n2.2.2.2").unwrap();

        // Setup gzip file
        let mut gzip_file = NamedTempFile::new().unwrap();
        flate2::write::GzEncoder::new(gzip_file.as_file_mut(), flate2::Compression::default())
            .write_all(b"1.1.1.1\n2.2.2.2")
            .unwrap();

        // Define subtests
        let tests = vec![
            (Path::new(""), FileType::NotAFile),
            (plain_text_file.path(), FileType::Plain),
            (gzip_file.path(), FileType::Gzip),
        ];

        // Run tests
        for test in tests {
            let actual = determine_file_type(Path::new(test.0));
            assert_eq!(test.1, actual.unwrap());
        }
    }
}
