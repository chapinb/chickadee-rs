use lazy_static::lazy_static;
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get_distinct_ips(ips: Vec<IpAddr>) -> Vec<IpAddr> {
    let mut distinct_ips = vec![];
    for ip in ips.into_iter() {
        if !distinct_ips.contains(&ip) {
            distinct_ips.push(ip);
        }
    }
    distinct_ips
}

pub fn get_routable_ips(ips: Vec<IpAddr>) -> Vec<IpAddr> {
    // TODO Filter out common BOGON
    ips.into_iter()
        .filter(|x| !x.is_unspecified() && !x.is_multicast() && !x.is_loopback())
        .collect()
}

pub fn get_all_ips(data: &str) -> Vec<IpAddr> {
    let mut all_ips: Vec<IpAddr> = get_all_ipv4(data);
    all_ips.append(&mut get_all_ipv6(data));
    all_ips
}

pub fn get_all_ipv4(data: &str) -> Vec<IpAddr> {
    lazy_static! {
        static ref IPV4: Regex = match Regex::new(
            r"(?:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9]))"
        ) {
            Ok(x) => x,
            Err(e) => panic!("Unanble to generate IPv4 pattern: {}", e),
        };
    }
    IPV4.find_iter(data)
        .map(|x| {
            let substr = x.as_str();
            substr.parse::<Ipv4Addr>().ok()
        })
        .filter(|x| x.is_some())
        .map(|x| IpAddr::V4(x.unwrap())) // It is safe to use unwrap, as we are filtering .is_some() above
        .collect::<Vec<_>>()
}

fn get_all_ipv6(data: &str) -> Vec<IpAddr> {
    lazy_static! {
        static ref IPV6: Regex = match Regex::new(
            r"(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,4}:[^\s:](?:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])))|(?:::(?:ffff(?::0{1,4}){0,1}:){0,1}[^\s:](?:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])))|(?:fe80:(?::(?:(?:[0-9a-fA-F]){1,4})){0,4}%[0-9a-zA-Z]{1,})|(?::(?:(?::(?:(?:[0-9a-fA-F]){1,4})){1,7}|:))|(?:(?:(?:[0-9a-fA-F]){1,4}):(?:(?::(?:(?:[0-9a-fA-F]){1,4})){1,6}))|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,2}(?::(?:(?:[0-9a-fA-F]){1,4})){1,5})|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,3}(?::(?:(?:[0-9a-fA-F]){1,4})){1,4})|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,4}(?::(?:(?:[0-9a-fA-F]){1,4})){1,3})|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,5}(?::(?:(?:[0-9a-fA-F]){1,4})){1,2})|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,6}:(?:(?:[0-9a-fA-F]){1,4}))|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){1,7}:)|(?:(?:(?:(?:[0-9a-fA-F]){1,4}):){7,7}(?:(?:[0-9a-fA-F]){1,4}))"
        ) {
            Ok(x) => x,
            Err(e) => panic!("Unanble to generate IPv6 pattern: {}", e),
        };
    }

    IPV6.find_iter(data)
        .map(|x| {
            let substr = x.as_str();
            substr.parse::<Ipv6Addr>().ok()
        })
        .filter(|x| x.is_some())
        .map(|x| IpAddr::V6(x.unwrap())) // It is safe to use unwrap, as we are filtering .is_some() above
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_routable_ips() {
        let tests = vec![(
            vec![
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0)),
            ],
            vec![IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))],
        )];

        for test in tests {
            let actual = get_routable_ips(test.0);
            assert_eq!(test.1, actual);
        }
    }

    #[test]
    fn test_get_distinct_ips() {
        let tests = vec![(
            vec![
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                IpAddr::V4(Ipv4Addr::new(10, 100, 27, 43)),
                IpAddr::V4(Ipv4Addr::new(10, 100, 27, 43)),
            ],
            vec![
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                IpAddr::V4(Ipv4Addr::new(10, 100, 27, 43)),
            ],
        )];

        for test in tests {
            let actual = get_distinct_ips(test.0);
            assert_eq!(test.1, actual);
        }
    }

    #[test]
    fn test_get_all_ipv4() {
        let tests = vec![
            ("8.8.8.8", vec![IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))]),
            (
                "some127.0.0.1data8.8.8.8here10.100.27.43",
                vec![
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                    IpAddr::V4(Ipv4Addr::new(10, 100, 27, 43)),
                ],
            ),
        ];
        for test in tests {
            let actual = get_all_ipv4(test.0);
            let expected = test.1;
            assert_eq!(expected, actual)
        }
    }

    #[test]
    fn test_get_all_ipv6() {
        let tests = vec![
            (
                "2001:4860:4860::8844",
                vec![IpAddr::V6(Ipv6Addr::new(
                    0x2001, 0x4860, 0x4860, 0x0, 0x0, 0x0, 0x0, 0x8844,
                ))],
            ),
            (
                "2620:119:53::53,2606:4700:4700::111",
                vec![
                    IpAddr::V6(Ipv6Addr::new(0x2620, 0x119, 0x53, 0x0, 0x0, 0x0, 0x0, 0x53)),
                    IpAddr::V6(Ipv6Addr::new(
                        0x2606, 0x4700, 0x4700, 0x0, 0x0, 0x0, 0x0, 0x111,
                    )),
                ],
            ),
        ];
        for test in tests {
            let actual = get_all_ipv6(test.0);
            let expected = test.1;
            assert_eq!(expected, actual)
        }
    }
}
