mod cli;
mod config;

use std::time::Duration;
pub use self::config::ConfigReadError;

#[derive(Default)]
pub struct Settings {
    interval: Option<Duration>,
    step: Option<i8>,
    blacklist: Option<Vec<String>>,
    whitelist: Option<Vec<String>>,
}

impl Settings {

    pub fn from_cli() -> Settings
    {
        cli::parse()
    }

    pub fn from_config() -> Result<Settings, ConfigReadError>
    {
        config::parse()
    }

    pub fn amend(&mut self, s: Settings) {
        if s.step.is_some() {
            self.step = s.step;
        }

        if s.interval.is_some() {
            self.interval = s.interval;
        }

        if s.blacklist.is_some() {
            self.blacklist = s.blacklist;
        }

        if s.whitelist.is_some() {
            self.whitelist = s.whitelist;
        }
    }

    pub fn get_interval(&self) -> Duration
    {
        self.interval.unwrap_or(Duration::from_secs(5))
    }

    pub fn get_step(&self) -> i8
    {
        self.step.unwrap_or(1)
    }

    pub fn get_blacklist(&self) -> Vec<String>
    {
        match self.blacklist.as_ref() {
            Some(x) => x.to_vec(),
            None => Vec::new()
        }
    }

    pub fn get_whitelist(&self) -> Vec<String>
    {
        match self.whitelist.as_ref() {
            Some(x) => x.to_vec(),
            None => Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings()
    {
        let settings = Settings{whitelist: Some(vec!(String::from("autonice"))), ..Default::default()};
        assert_eq!(1, settings.get_step());
        assert_eq!(Duration::from_secs(5), settings.get_interval());
        assert_eq!(0, settings.get_blacklist().len());

        let whitelist = settings.get_whitelist();
        assert_eq!(1, whitelist.len());
        assert!(whitelist.contains(&String::from("autonice")));
    }

    #[test]
    fn test_amend_settings()
    {
        let mut settings = Settings{interval: Some(Duration::from_secs(12)), step: Some(3), ..Default::default()};
        let amend = Settings{step: Some(5), whitelist: Some(vec!(String::from("autonice"))), ..Default::default()};

        settings.amend(amend);
        assert_eq!(settings.get_interval().as_secs(), 12);
        assert_eq!(settings.get_step(), 5);
        assert_eq!(settings.get_whitelist().len(), 1);
    }
}
