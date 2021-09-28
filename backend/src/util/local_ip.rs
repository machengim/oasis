use anyhow::Result as AnyResult;
use std::net::{IpAddr, Ipv4Addr};

#[allow(dead_code)]
struct LocalIpRange {
    start: IpAddr,
    end: IpAddr,
}

impl LocalIpRange {
    #[allow(dead_code)]
    fn new(ips: [u8; 4], ipe: [u8; 4]) -> Self {
        let start = IpAddr::V4(Ipv4Addr::new(ips[0], ips[1], ips[2], ips[3]));
        let end = IpAddr::V4(Ipv4Addr::new(ipe[0], ipe[1], ipe[2], ipe[3]));

        LocalIpRange { start, end }
    }
}

pub fn get() -> AnyResult<IpAddr> {
    #[cfg(target_os = "linux")]
    {
        match local_ip_address::local_ip() {
            Ok(ip) => Ok(ip),
            Err(_) => Err(anyhow::anyhow!("Cannot retreive local ip address")),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        use std::cmp::Ordering;

        let mut ranges = vec![];
        ranges.push(LocalIpRange::new([192, 168, 0, 0], [192, 168, 255, 255]));
        ranges.push(LocalIpRange::new([172, 16, 0, 0], [172, 31, 255, 255]));
        ranges.push(LocalIpRange::new([10, 0, 0, 0], [10, 255, 255, 255]));

        let names = vec!["ethernet", "wi-fi", "en0"];
        let network_interfaces = local_ip_address::list_afinet_netifas().unwrap();

        for (name, ip) in network_interfaces.iter() {
            if !ip.is_ipv4() {
                continue;
            }

            for range in ranges.iter() {
                if ip.cmp(&range.start) == Ordering::Greater
                    && ip.cmp(&range.end) == Ordering::Less
                    && names.contains(&name.to_lowercase().as_str())
                {
                    return Ok(*ip);
                }
            }
        }

        Err(anyhow::anyhow!("Cannot retrieve local ip"))
    }
}
