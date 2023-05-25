use std::net::IpAddr;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct VTResolver;

#[derive(Deserialize, Serialize, Debug)]
pub struct Resolution {
    hostname: String,
    last_resolved: String,
}

//enum for asn field as VT API indicates response can be either string or int
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum StringOrInt {
    String(String),
    Int(i32),
}

//raw struct and regular struct for getting usize
#[derive(Deserialize, Debug)]
struct RawVirusTotalRecord {
    response_code: Option<i32>,
    verbose_msg: Option<String>,
    asn: Option<StringOrInt>,
    country: Option<String>,
    resolutions: Option<Vec<Resolution>>,
    detected_urls: Option<Vec<Value>>,
    detected_downloaded_samples: Option<Vec<Value>>,
    undetected_downloaded_samples: Option<Vec<Value>>,
    undetected_urls: Option<Vec<Value>>,
}

#[derive(Serialize, Debug)]
pub struct VirusTotalRecord {
    response_code: Option<i32>,
    verbose_msg: Option<String>,
    asn: Option<StringOrInt>,
    country: Option<String>,
    resolutions: Option<Vec<Resolution>>,
    detected_url_count: usize,
    detected_downloaded_sample_count: usize,
    undetected_downloaded_sample_count: usize,
    undetected_url_count: usize,
}

impl VTResolver {
    pub fn new() -> Self {
        Self
    }

    fn request(ip: IpAddr) -> anyhow::Result<String> {
        let api_key = std::env::var("VIRUSTOTAL_API_KEY")
            .map_err(|_| anyhow::Error::msg("VIRUSTOTAL_API_KEY not set in environment"))?;
        let url = format!("https://www.virustotal.com/vtapi/v2/ip-address/report?apikey={}&ip={}", api_key, ip);
        let client = Client::new();
        let response = client.get(&url).send()?;
        Ok(response.text()?)
    }

    fn parse(response: &str) -> anyhow::Result<VirusTotalRecord> {
        let raw_record: RawVirusTotalRecord = serde_json::from_str(response)?;
        let vt_record = VirusTotalRecord {
            response_code: raw_record.response_code,
            verbose_msg: raw_record.verbose_msg,
            asn: raw_record.asn,
            country: raw_record.country,
            resolutions: raw_record.resolutions,
            detected_url_count: raw_record.detected_urls.map_or(0, |v| v.len()),
            detected_downloaded_sample_count: raw_record.detected_downloaded_samples.map_or(0, |v| v.len()),
            undetected_downloaded_sample_count: raw_record.undetected_downloaded_samples.map_or(0, |v| v.len()),
            undetected_url_count: raw_record.undetected_urls.map_or(0, |v| v.len()),
        };
        Ok(vt_record)
    }

    pub fn resolve(&self, ip_addresses: Vec<IpAddr>) -> anyhow::Result<Vec<String>> {
        let results = Vec::new();
        for ip in ip_addresses {
            let response = Self::request(ip)?;
            let vt_record = Self::parse(&response)?;

            println!("VirusTotal Record for: {:?}", ip);
            println!("Response code: {:?}", vt_record.response_code);
            println!("Verbose message: {:?}", vt_record.verbose_msg);
            println!("ASN: {:?}", vt_record.asn);
            println!("Country: {:?}", vt_record.country);
            println!("Detected URL count: {:?}", vt_record.detected_url_count);
            println!("Detected downloaded sample count: {:?}", vt_record.detected_downloaded_sample_count);
            println!("Undetected downloaded sample count: {:?}", vt_record.undetected_downloaded_sample_count);
            println!("Undetected URL count: {:?}", vt_record.undetected_url_count);

            if let Some(resolutions) = &vt_record.resolutions {
                if !resolutions.is_empty() {
                    for resolution in resolutions.iter().take(5) {
                        println!("{:?}", resolution);
                    }
                } else {
                    println!("No resolutions found");
                }
            }

            println!();
        }
        Ok(results)
    }
}