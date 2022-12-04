use std::fs;
use std::net::IpAddr;
use std::path::Path;

pub fn parse_text_file(file_path: Path) -> Vec<IpAddr> {
    let file_content = fs::read_to_string(file_path.to_str());
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfilei::tempfile;

    fn test_parse_text_file() {
        let tests = vec![
            ("8.8.8.8", vec![]),
            ("10.0.0.1,10.0.2.1", vec![]),
            ("1.1.1.1,sample,127.0.0.1,data", vec![]),
        ];
        for test in tests {
            let mut sample = tempfile();
            writeln!(test.0);

        }
    }
}
