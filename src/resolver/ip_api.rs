use anyhow::Result;
use serde_json::Value;
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

impl IpApiRecord {
    fn from_value(value: &Value) -> IpApiRecord {
        IpApiRecord {
            query: value.get("query").and_then(|v| v.as_str()).map(String::from),
            status: value.get("status").and_then(|v| v.as_str()).map(String::from),
            continent: value.get("continent").and_then(|v| v.as_str()).map(String::from),
            continent_code: value.get("continentCode").and_then(|v| v.as_str()).map(String::from),
            country: value.get("country").and_then(|v| v.as_str()).map(String::from),
            country_code: value.get("countryCode").and_then(|v| v.as_str()).map(String::from),
            region: value.get("region").and_then(|v| v.as_str()).map(String::from),
            region_name: value.get("regionName").and_then(|v| v.as_str()).map(String::from),
            city: value.get("city").and_then(|v| v.as_str()).map(String::from),
            district: value.get("district").and_then(|v| v.as_str()).map(String::from),
            zip: value.get("zip").and_then(|v| v.as_str()).map(String::from),
            lat: value.get("lat").and_then(|v| v.as_f64()).map(|v| v as f32),
            lon: value.get("lon").and_then(|v| v.as_f64()).map(|v| v as f32),
            timezone: value.get("timezone").and_then(|v| v.as_str()).map(String::from),
            offset: value.get("offset").and_then(|v| v.as_u64()).map(|v| v as u32),
            currency: value.get("currency").and_then(|v| v.as_str()).map(String::from),
            isp: value.get("isp").and_then(|v| v.as_str()).map(String::from),
            org: value.get("org").and_then(|v| v.as_str()).map(String::from),
            asn: value.get("as").and_then(|v| v.as_str()).map(String::from),
            asname: value.get("asname").and_then(|v| v.as_str()).map(String::from),
            mobile: value.get("mobile").and_then(|v| v.as_bool()),
            proxy: value.get("proxy").and_then(|v| v.as_bool()),
            hosting: value.get("hosting").and_then(|v| v.as_bool()),
        }
    }
}

pub struct Resolver {
    pub columns: Vec<String>,
}

impl Resolver {
    pub fn new(columns: Option<Vec<String>>) -> Resolver {
        // If provided columns, use those. Otherwise, use all.
        match columns {
            Some(columns) => {
                let allowed_columns = Resolver::check_columns(columns);
                Resolver { columns: allowed_columns }
            },
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
            String::from("continentCode"),
            String::from("country"),
            String::from("countryCode"),
            String::from("region"),
            String::from("regionName"),
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
            String::from("as"),
            String::from("asname"),
            String::from("mobile"),
            String::from("proxy"),
            String::from("hosting"),
        ]
    }

    pub fn resolve(&self, ips: Vec<IpAddr>) -> Result<IpApiRecords> {
        let mut all_responses = IpApiRecords::new();
        for ip_addr in ips {
            let mut url = format!("http://ip-api.com/json/{}", ip_addr);

            if self.columns != Resolver::allowed_columns() {
                // Add fields to the URL, if we don't have all fields set
                let mut fields = "".to_string();
                for column in self.columns.clone() {
                    fields.push_str(&format!("{},", column));
                }
                url.push_str(&format!("?fields={}", fields.trim_end_matches(',')));
            }

            println!("{}", url);

            let resp = reqwest::blocking::get(url)?.error_for_status()?;

            match resp.json::<Value>() {
                Ok(record) => {
                    let record = IpApiRecord::from_value(&record);
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
