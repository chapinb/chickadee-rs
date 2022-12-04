use std::fs;
use std::net::IpAddr;
use std::path::Path;

use crate::util::get_all_ips;

pub fn parse_text_file(file_path: &Path) -> Vec<IpAddr> {
    let file_content = fs::read_to_string(&file_path.to_str().unwrap());
    get_all_ips(file_content.unwrap().as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_text_file() {
        let tests = vec![
            ("8.8.8.8", vec![IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))]),
            (
                "10.0.0.1,10.0.2.1",
                vec![
                    IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                    IpAddr::V4(Ipv4Addr::new(10, 0, 2, 1)),
                ],
            ),
            (
                "1.1.1.1,sample,127.0.0.1,data,2001:4860:4860::8844,foo",
                vec![
                    IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    IpAddr::V6(Ipv6Addr::new(
                        0x2001, 0x4860, 0x4860, 0x0, 0x0, 0x0, 0x0, 0x8844,
                    )),
                ],
            ),
        ];
        for test in tests {
            let mut sample = NamedTempFile::new().unwrap();
            writeln!(sample, "{}", test.0).unwrap();
            let actual = parse_text_file(sample.path());
            let expected = test.1;
            assert_eq!(expected, actual);
        }
    }
}
