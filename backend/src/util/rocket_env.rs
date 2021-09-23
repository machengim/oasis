use std::collections::HashMap;

// use this instead of `dotenv` because of the conflict
// when double clicking the app to launch in MacOS,
// dotenv doesn't work properly.
pub struct RocketEnv<'a> {
    pub vars: HashMap<&'a str, &'a str>,
}

impl<'a> RocketEnv<'a> {
    #[cfg(not(debug_assertions))]
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("ROCKET_ADDRESS", "0.0.0.0");
        vars.insert("ROCKET_PORT", "8000");
        vars.insert("ROCKET_WORKERS", "8");
        vars.insert("ROCKET_KEEP_ALIVE", "2");
        vars.insert("ROCKET_LOG_LEVEL", "off");

        RocketEnv { vars }
    }

    #[cfg(debug_assertions)]
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        vars.insert("ROCKET_ADDRESS", "127.0.0.1");
        vars.insert("ROCKET_PORT", "8000");
        vars.insert("ROCKET_WORKERS", "2");
        vars.insert("ROCKET_KEEP_ALIVE", "1");
        vars.insert("ROCKET_LOG_LEVEL", "critical");

        RocketEnv { vars }
    }

    pub fn setup(&self) {
        for (key, value) in &self.vars {
            std::env::set_var(key, value);
        }
    }
}
