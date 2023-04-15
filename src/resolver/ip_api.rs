use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Deserialize, Serialize, Default)]
pub struct IpApiRecords {
    pub records: Vec<IpApiRecord>,
}
impl IpApiRecords {
    pub fn new() -> IpApiRecords {
        IpApiRecords { records: vec![] }
    }
}

#[derive(Deserialize, Serialize)]
pub struct IpApiRecord {
    pub query: Option<String>,
    pub status: Option<String>,
    pub continent: Option<String>,
    pub continent_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub region_name: Option<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub zip: Option<String>,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub timezone: Option<String>,
    pub offset: Option<u32>,
    pub currency: Option<String>,
    pub isp: Option<String>,
    pub org: Option<String>,
    pub asn: Option<String>,
    pub asname: Option<String>,
    pub mobile: Option<bool>,
    pub proxy: Option<bool>,
    pub hosting: Option<bool>,
}

pub struct Resolver {
    pub columns: Vec<String>,
}

impl Resolver {
    pub fn new(columns: Option<Vec<String>>) -> Resolver {
        // If provided columns, use those. Otherwise, use all.
        match columns {
            Some(columns) => Resolver { columns },
            None => Resolver {
                columns: Resolver::allowed_columns(),
            },
        }
    }

    fn check_columns(requested_columns: Vec<String>) -> Vec<String> {
        let allowed = Resolver::allowed_columns();
        requested_columns
            .into_iter()
            .filter(|column| allowed.contains(column))
            .collect()
    }

    fn allowed_columns() -> Vec<String> {
        vec![
            String::from("query"),
            String::from("status"),
            String::from("continent"),
            String::from("continent_code"),
            String::from("country"),
            String::from("country_code"),
            String::from("region"),
            String::from("region_name"),
            String::from("city"),
            String::from("district"),
            String::from("zip"),
            String::from("lat"),
            String::from("lon"),
            String::from("timezone"),
            String::from("offset"),
            String::from("currency"),
            String::from("isp"),
            String::from("org"),
            String::from("asn"),
            String::from("asname"),
            String::from("mobile"),
            String::from("proxy"),
            String::from("hosting"),
        ]
    }

    pub fn resolve(&self, ips: Vec<IpAddr>) -> Result<IpApiRecords> {
        let mut all_responses = IpApiRecords::new();
        for ip_addr in ips {
            let url = format!("http://ip-api.com/json/{}", ip_addr);
            let resp = reqwest::blocking::get(url)?.error_for_status()?;
            match resp.json::<IpApiRecord>() {
                Ok(record) => {
                    all_responses.records.push(record);
                }
                Err(e) => {
                    eprintln!("Error resolving IP: {} {}", ip_addr, e);
                }
            }
        }
        Ok(all_responses)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_resolve_single_ip() {
        let resolver = Resolver::new(Some(vec![String::from("query"), String::from("city")]));
        let ips = vec![IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))];
        let res = resolver.resolve(ips);

        assert!(res.is_ok());

        let resolved = res.unwrap();

        assert_eq!(1, resolved.records.len());
    }

    #[test]
    fn test_cols_allowed() {
        let sample = vec![
            String::from("query"),
            String::from("status"),
            String::from("lat"),
            String::from("lon"),
        ];
        let actual = Resolver::check_columns(sample.to_owned());
        assert_eq!(sample, actual);
    }
}
