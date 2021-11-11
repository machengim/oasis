use super::local_ip::ServerConfig;
use std::collections::HashMap;

// use this instead of `dotenv` because of the conflict
// when double clicking the app to launch in MacOS,
// dotenv doesn't work properly.
pub struct RocketEnv<'a> {
    pub vars: HashMap<&'a str, &'a str>,
}

impl<'a> RocketEnv<'a> {
    #[cfg(not(debug_assertions))]
    pub fn setup(config: &ServerConfig) {
        let ip_str = config.ip.to_string();
        let port_str = config.port.to_string();
        let tls_str;

        let mut vars = HashMap::new();
        vars.insert("ROCKET_ADDRESS", &ip_str[..]);
        vars.insert("ROCKET_PORT", &port_str[..]);
        vars.insert("ROCKET_WORKERS", "8");
        vars.insert("ROCKET_KEEP_ALIVE", "4");
        vars.insert("ROCKET_LOG_LEVEL", "off");
        vars.insert("ROCKET_LIMITS", "{file=\"8 MiB\"}");
        if config.certs.is_some() && config.key.is_some() {
            tls_str = config.get_tls_str();
            vars.insert("ROCKET_TLS", &tls_str);
        }

        let env = RocketEnv { vars };
        env.set_var();
    }

    #[cfg(debug_assertions)]
    pub fn setup(config: &ServerConfig) {
        let ip_str = config.ip.to_string();
        let port_str = config.port.to_string();
        let tls_str;

        let mut vars = HashMap::new();
        vars.insert("ROCKET_ADDRESS", &ip_str[..]);
        vars.insert("ROCKET_PORT", &port_str[..]);
        vars.insert("ROCKET_WORKERS", "2");
        vars.insert("ROCKET_KEEP_ALIVE", "1");
        vars.insert("ROCKET_LOG_LEVEL", "debug");
        vars.insert("ROCKET_LIMITS", "{file=\"8 MiB\"}");
        if config.certs.is_some() && config.key.is_some() {
            tls_str = config.get_tls_str();
            vars.insert("ROCKET_TLS", &tls_str);
        }

        let env = RocketEnv { vars };
        env.set_var();
    }

    fn set_var(&self) {
        for (key, value) in &self.vars {
            std::env::set_var(key, value);
        }
    }
}
