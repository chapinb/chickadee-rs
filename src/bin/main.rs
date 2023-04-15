use std::{net::IpAddr};
use serde_json;
use libchickadee::resolver::ip_api::{IpApiRecords, Resolver};

fn resolve_ip_addresses(ip_addresses: Vec<IpAddr>) -> Vec<String> {
    let ip_records = Resolver::new(None).resolve(ip_addresses).unwrap();
    ip_records.records.iter().map(|record| serde_json::to_string(record).unwrap()).collect()
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_resolve_ip_addresses() {
        let ip_addresses = vec![
            IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
        ];
        let ip_records = resolve_ip_addresses(ip_addresses);

        assert_eq!(1, ip_records.len());
        assert!(ip_records[0].contains("country_code"));
        assert!(ip_records[0].contains("1.1.1.1"));
    }
}
