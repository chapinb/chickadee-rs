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

pub struct Resolve {
    pub ips: Vec<IpAddr>,
    results: IpApiRecords,
    client: Client,
}

#[cfg(test)]
mod test {
    use super::*;
}
