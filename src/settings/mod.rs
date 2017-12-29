mod cli;

use std::time::Duration;

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
        let settings = Settings{interval: None, step: None, blacklist: None, whitelist: Some(vec!(String::from("autonice")))};
        assert_eq!(1, settings.get_step());
        assert_eq!(Duration::from_secs(5), settings.get_interval());
        assert_eq!(0, settings.get_blacklist().len());

        let whitelist = settings.get_whitelist();
        assert_eq!(1, whitelist.len());
        assert!(whitelist.contains(&String::from("autonice")));
    }
}
