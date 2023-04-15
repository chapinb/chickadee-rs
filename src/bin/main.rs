use clap::{self, Parser};
use libchickadee::resolver::ip_api::Resolver;
use std::net::IpAddr;

fn resolve_ip_addresses(ip_addresses: Vec<IpAddr>, columns: Option<Vec<String>>) -> Vec<String> {
    let ip_records = Resolver::new(columns).resolve(ip_addresses).unwrap();
    ip_records
        .records
        .iter()
        .map(|record| serde_json::to_string(record).unwrap())
        .collect()
}

// Create new struct for Clap to parse CLI arguments
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Get the IP addresses to resolve from CLI arguments using clap
    #[clap(long)]
    ip: IpAddr,

    /// Allow user to specify which columns to use
    #[clap(long)]
    columns: Option<String>,
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Resolve IP addresses
    let ip_records = resolve_ip_addresses(vec![cli.ip],  cli.columns.map(|s| s.split(',').map(|s| s.to_string()).collect()));

    // Print IP records
    for ip_record in ip_records {
        println!("{}", ip_record);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_resolve_ip_addresses() {
        let ip_addresses = vec![IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))];
        let columns = Some(vec!["country_code".to_string(), "query".to_string()]);
        let ip_records = resolve_ip_addresses(ip_addresses, columns);

        assert_eq!(1, ip_records.len());
        assert!(ip_records[0].contains("country_code"));
        assert!(ip_records[0].contains("1.1.1.1"));
    }
}
