use crate::util;
use crate::util::constants::DEFAULT_IP;
use anyhow::Result as AnyResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[derive(Debug)]
pub struct ServerConfig {
    pub ip: IpAddr,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ip: DEFAULT_IP,
            port: 8000,
        }
    }
}

impl ServerConfig {
    pub fn new() -> AnyResult<Self> {
        let mut server = Self::default();
        let malform = anyhow::anyhow!("Malformed config file");
        let conf_file = util::get_pwd().join("oasis.conf");

        if conf_file.exists() && conf_file.is_file() {
            let file = File::open(conf_file)?;
            let lines = BufReader::new(file).lines();
            for line in lines {
                if let Ok(content) = line {
                    if content.starts_with("#") {
                        continue;
                    }

                    let parts: Vec<&str> = content.split("=").collect();
                    if parts.len() != 2 {
                        return Err(malform);
                    }

                    match parts[0].trim() {
                        "ip" => server.ip = IpAddr::from_str(parts[1].trim())?,
                        "port" => server.port = parts[1].trim().parse()?,
                        _ => return Err(malform),
                    }
                }
            }
        }

        Ok(server)
    }
}

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

pub fn show(config: &ServerConfig) -> AnyResult<()> {
    let (ip, multiple) = if config.ip == DEFAULT_IP {
        retrieve_ip()?
    } else {
        (config.ip, false)
    };

    if multiple {
        println!(
            "Multiple IPs detected, please visit your server via its ip and port {}",
            config.port
        );
        println!("You can also specify the server ip in the `oasis.conf` file");
    } else {
        println!("Server running on {}:{}", ip, config.port);
    }

    Ok(())
}

fn retrieve_ip() -> AnyResult<(IpAddr, bool)> {
    #[cfg(target_os = "linux")]
    {
        match local_ip_address::local_ip() {
            Ok(ip) => Ok((ip, false)),
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

        // the name is not sure, it could be "wlan" or "以太网" on some devices.
        // let names = vec!["ethernet", "wi-fi", "en0"];
        let network_interfaces = local_ip_address::list_afinet_netifas().unwrap();
        let ips = vec![];
        for (_name, ip) in network_interfaces.iter() {
            if !ip.is_ipv4() {
                continue;
            }

            for range in ranges.iter() {
                if ip.cmp(&range.start) == Ordering::Greater && ip.cmp(&range.end) == Ordering::Less
                // && names.contains(&name.to_lowercase().as_str())
                {
                    ips.push(*ip);
                }
            }
        }

        match ips.len() {
            0 => Err(anyhow::anyhow!("Cannot retrieve local ip")),
            1 => Ok((ips[0], false)),
            _ => Ok((ips[0], true)),
        }
    }
}
