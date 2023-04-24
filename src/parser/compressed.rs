use crate::util::get_all_ips;
use anyhow::Result;
use flate2;
use std::io::Read;
use std::net::IpAddr;
use std::path::Path;

pub fn parse_gzip_file(file_path: &Path) -> Result<Vec<IpAddr>> {
    let mut file = flate2::read::GzDecoder::new(std::fs::File::open(file_path)?);
    let mut buffer = Vec::with_capacity(8192);
    file.read_to_end(&mut buffer).unwrap();

    // Convert the buffer to a str
    let contents = String::from_utf8(buffer)?;
    Ok(get_all_ips(contents.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::Compression;
    use std::io::Write;
    use std::net::Ipv4Addr;
    use tempfile::NamedTempFile;

    #[test]
    fn test_gunzip() {
        // Setup test data
        let file_path = NamedTempFile::new().unwrap();
        let mut file = flate2::write::GzEncoder::new(
            std::fs::File::create(file_path.path()).unwrap(),
            Compression::default(),
        );
        file.write_all(b"1.1.1.1\n2.2.2.2").unwrap();
        file.finish().unwrap();

        // Run test
        let res = parse_gzip_file(file_path.path());
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(
            vec![
                IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                IpAddr::V4(Ipv4Addr::new(2, 2, 2, 2))
            ],
            res
        );
    }
}
