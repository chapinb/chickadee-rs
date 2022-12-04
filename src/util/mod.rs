use lazy_static::lazy_static;
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn get_all_ipv4(data: &str) -> Vec<Ipv4Addr> {
    lazy_static! {
        static ref IPV4: Regex = Regex::new(
            r"(?:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9]))"
        ).unwrap();
    }
    IPV4.find_iter(data)
        .map(|x| {
            println!("{x:#?}");
            let substr = x.as_str();
            substr.parse::<Ipv4Addr>().ok()
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_all_ipv4() {
        let tests = vec![
            ("a8.8.8.8a", vec![Ipv4Addr::new(8, 8, 8, 8)]),
            (
                "some127.0.0.1data8.8.8.8here10.100.27.43",
                vec![
                    Ipv4Addr::new(127, 0, 0, 1),
                    Ipv4Addr::new(8, 8, 8, 8),
                    Ipv4Addr::new(10, 100, 27, 43),
                ],
            ),
        ];
        for test in tests {
            let actual = get_all_ipv4(test.0);
            let expected = test.1;
            assert_eq!(expected, actual)
        }
    }
}
