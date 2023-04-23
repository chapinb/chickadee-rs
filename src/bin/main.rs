use anyhow::Result;
use clap::{self, Parser};
use libchickadee::{parser::plain::parse_text_file, resolver::ip_api::Resolver, util::get_all_ips};
use std::{net::IpAddr, path::Path};

fn resolve_ip_addresses(ip_addresses: Vec<IpAddr>, columns: Option<Vec<String>>) -> Vec<String> {
    let ip_records = Resolver::new(columns).resolve(ip_addresses).unwrap();
    ip_records
        .records
        .iter()
        .map(|record| serde_json::to_string(record).unwrap())
        .collect()
}

struct Extractor {
    // Extract IP address from input
    source: String,
    is_file: bool,
}

impl Extractor {
    fn new(source: String) -> Self {
        let is_file = Path::new(&source).exists();
        Self { source, is_file }
    }

    fn extract(&self) -> Result<Vec<IpAddr>> {
        if self.is_file {
            Ok(parse_text_file(Path::new(&self.source)))
        } else {
            Ok(get_all_ips(self.source.as_str()))
        }
    }
}

// Create new struct for Clap to parse CLI arguments
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Get the IP addresses to resolve from CLI arguments using clap.
    /// This may be a delimited string or a file path to a text file
    /// containing one or more IP addresses.
    #[clap(long)]
    ips: String,

    /// Allow user to specify which columns to select in the GeoIP resolution.
    /// Currently only supports the columns found in ip-api.com
    #[clap(long)]
    columns: Option<String>,
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Extract IP addresses
    let extractor = Extractor::new(cli.ips);
    let ip_addresses = extractor.extract().unwrap();

    // Resolve IP addresses
    let ip_records = resolve_ip_addresses(
        ip_addresses,
        cli.columns
            .map(|s| s.split(',').map(|s| s.to_string()).collect()),
    );

    // Print IP records
    for ip_record in ip_records {
        println!("{}", ip_record);
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;
    use std::{io::Write, net::Ipv4Addr};

    #[test]
    fn test_resolve_ip_addresses() {
        let ip_addresses = vec![IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))];
        let columns = Some(vec!["country_code".to_string(), "query".to_string()]);
        let ip_records = resolve_ip_addresses(ip_addresses, columns);

        assert_eq!(1, ip_records.len());
        assert!(ip_records[0].contains("country_code"));
        assert!(ip_records[0].contains("1.1.1.1"));
    }

    #[test]
    fn test_resolve_ip_addresses_from_file() {
        // Create a new temporary text file
        let mut temp_path = NamedTempFile::new().unwrap();
        temp_path
            .write_fmt(format_args!(
                "{},{},\t{}\n{}",
                "1.1.1.1", "2.2.2.2", "4", "3.3.3.3"
            ))
            .unwrap();

        // Pass this file into
        let extractor = Extractor::new(temp_path.path().to_string_lossy().to_string());
        let ip_addresses = extractor.extract();

        assert!(ip_addresses.is_ok());
        assert!(ip_addresses.unwrap().len() == 3);
    }
}
