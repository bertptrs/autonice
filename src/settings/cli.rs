extern crate clap;

use self::clap::App;
use std::env;
use std::ffi::OsString;
use std::time::Duration;
use super::Settings;

pub fn parse() -> Settings
{
    return from_iter(env::args());
}

fn from_iter<I: IntoIterator<Item = T>, T: Into<OsString> + Clone>(iter: I) -> Settings
{
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches_from(iter);

    let step = match matches.value_of("step") {
        Some(x) => Some(x.parse::<i8>().expect("Invalid value for step")),
        None => None
    };

    let interval = match matches.value_of("interval") {
        Some(x) => Some(Duration::from_secs(x.parse::<u64>().expect("Invalid value for interval"))),
        None => None
    };

    let blacklist = matches.values_of_lossy("blacklist");
    let whitelist = matches.values_of_lossy("whitelist");

    Settings{step, blacklist, whitelist, interval}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse()
    {
        let settings = from_iter(vec!["autonice", "-s", "12", "-i", "3", "-b", "systemd", "-b", "firefox", "autonice", "make", "smbclient"]);

        assert_eq!(settings.get_step(), 12);
        assert_eq!(settings.get_interval().as_secs(), 3);
        println!("{:?}", settings.get_whitelist());
        assert_eq!(settings.get_blacklist().len(), 2);
        assert_eq!(settings.get_whitelist().len(), 3);
    }
}
