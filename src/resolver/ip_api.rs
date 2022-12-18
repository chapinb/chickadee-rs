use reqwest::Client;
use std::net::IpAddr;

// TODO Make serializable with serde
pub struct IpApiRecords {
    pub records: Vec<IpApiRecord>,
}

pub struct IpApiRecord {
    pub query: String,
    pub status: String,
    pub continent: String,
    pub continent_code: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub region_name: String,
    pub city: String,
    pub district: String,
    pub zip: String,
    pub lat: f32,
    pub lon: f32,
    pub timezone: String,
    pub offset: u32,
    pub currency: String,
    pub isp: String,
    pub org: String,
    pub asn: String,
    pub asname: String,
    pub mobile: bool,
    pub proxy: bool,
    pub hosting: bool,
}

pub struct Resolver {
    pub ips: Vec<IpAddr>,
    pub columns: Vec<String>,
}

impl Resolver {
    fn new(ips: Vec<IpAddr>, columns: Vec<String>) -> Resolver {
        return Resolver {ips, columns};
    }

    fn check_columns(requested_columns: Vec<String>) -> Vec<String> {
        let allowed = Resolver::allowed_columns();
        let selected = requested_columns
            .into_iter()
            .filter(|column| allowed.contains(column))
            .collect();
        return selected;
    }

    fn allowed_columns() -> Vec<String> {
        return vec![
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
        ];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::Ipv4Addr;

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
