use anyhow::Result;
use clap::{self, Parser};
use libchickadee::parser::{
    compressed::parse_gzip_file, determine_file_type, plain::parse_text_file, SourceFileType,
};
use libchickadee::{resolver::ip_api::Resolver, util::get_all_ips};
use std::{net::IpAddr, path::Path};

fn resolve_ip_addresses(
    ip_addresses: Vec<IpAddr>,
    columns: Option<Vec<String>>,
) -> Result<Vec<String>> {
    let ip_records = Resolver::new(columns).resolve(ip_addresses)?;
    Ok(ip_records
        .records
        .iter()
        .filter_map(|record| serde_json::to_string(record).ok())
        .collect())
}

fn print_records(ip_records: Vec<String>, columns: Option<Vec<String>>) {
    for ip_record in ip_records {
        if let Some(ref cols) = columns {
            // If we have columns
            if let Ok(record) = ip_record.parse::<serde_json::Value>() {
                // And if we can parse the record as a JSON object
                // Setup a new filtered record
                let mut new_record = serde_json::Map::new();

                // Convert the record to a map
                let rec = match record.as_object() {
                    Some(rec) => rec,
                    None => {
                        eprintln!("Error parsing JSON object: {}", ip_record);
                        continue;
                    }
                };

                // Iterate over the columns and add them to the new record
                for (key, value) in rec {
                    if cols.contains(key) {
                        new_record.insert(key.to_string(), value.clone());
                    }
                }
                // Print the filtered record
                match serde_json::to_string(&new_record) {
                    Ok(record) => println!("{}", record),
                    Err(e) => eprintln!("Unable to display record: {}", e),
                }
            } else {
                // Print an error to stderr if we can't parse the record
                eprintln!("Error parsing JSON object: {}", ip_record);
                continue;
            }
        } else {
            // Print the whole record
            println!("{}", ip_record);
        }
    }
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
        if !self.is_file {
            // This must be a string input
            return Ok(get_all_ips(self.source.as_str()));
        }

        let source_path = Path::new(&self.source);

        match determine_file_type(source_path)? {
            SourceFileType::Plain => parse_text_file(source_path),
            SourceFileType::Gzip => parse_gzip_file(source_path),
            SourceFileType::NotAFile => Ok(get_all_ips(self.source.as_str())),
        }
    }
}

// Create new struct for Clap to parse CLI arguments
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// IP addresses to resolve from CLI arguments using clap.
    /// This may be a delimited string or a file path to a text file
    /// containing one or more IP addresses.
    #[clap(long)]
    ips: String,

    /// Specify which columns to select in the GeoIP resolution.
    /// Currently only supports the columns for ip-api.com.
    #[clap(long)]
    columns: Option<String>,
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();
    let columns = cli
        .columns
        .map(|s| s.split(',').map(|s| s.to_string()).collect());

    match run_chickadee(cli.ips, columns) {
        Ok(_) => (),
        Err(e) => eprintln!("Exiting due to error: {}", e),
    };
}

fn run_chickadee(ips: String, columns: Option<Vec<String>>) -> Result<()> {
    // Extract IP addresses
    let extractor = Extractor::new(ips.clone());
    let ip_addresses = match extractor.extract() {
        Ok(ip_addresses) => Ok(ip_addresses),
        Err(e) => {
            eprintln!("Error while extracting IP addresses from {}: {}", ips, e);
            Err(e)
        }
    }?;

    // Resolve IP addresses
    let ip_records = match resolve_ip_addresses(ip_addresses, columns.clone()) {
        Ok(ip_records) => Ok(ip_records),
        Err(e) => {
            eprintln!("Error during resolution: {}", e);
            Err(e)
        }
    }?;

    // Print IP records
    print_records(ip_records, columns);

    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;
    use std::{io::Write, net::Ipv4Addr};

    #[test]
    fn test_resolve_ip_addresses() {
        let ip_addresses = vec![IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))];
        let columns = Some(vec!["countryCode".to_string(), "query".to_string()]);
        let ip_records = resolve_ip_addresses(ip_addresses, columns);

        assert!(ip_records.is_ok());
        assert_eq!(1, ip_records.as_ref().unwrap().len());
        assert!(ip_records.as_ref().unwrap()[0].contains("countryCode"));
        assert!(ip_records.as_ref().unwrap()[0].contains("1.1.1.1"));
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

    #[test]
    fn test_run_chickadee_str() {
        let ips = "1.1.1.1,2.2.2.2\t3.3.3.3\n4.4.4.4".to_string();
        let columns = Some(vec!["countryCode".to_string(), "query".to_string()]);
        let res = run_chickadee(ips, columns);
        assert!(res.is_ok());
    }

    #[test]
    fn test_run_chickadee_file() {
        // Create a new temporary text file
        let mut temp_path = NamedTempFile::new().unwrap();
        temp_path
            .write_fmt(format_args!(
                "{},{},\t{}\n{}",
                "1.1.1.1", "2.2.2.2", "4", "3.3.3.3"
            ))
            .unwrap();

        let ips = temp_path.path().to_string_lossy().to_string();
        let columns = Some(vec!["countryCode".to_string(), "query".to_string()]);
        let res = run_chickadee(ips, columns);
        assert!(res.is_ok());
    }
}
