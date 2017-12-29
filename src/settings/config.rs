extern crate xdg_basedir;
extern crate yaml_rust;

use self::yaml_rust::{Yaml,YamlLoader,ScanError};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use super::Settings;

#[derive(Debug)]
pub enum ConfigReadError {
    FileReadError,
    InvalidYamlError,
    XDGError,
}

impl From<ScanError> for ConfigReadError {
    fn from(_: ScanError) -> ConfigReadError {
        ConfigReadError::InvalidYamlError
    }
}

impl From<::std::io::Error> for ConfigReadError {
    fn from(_: ::std::io::Error) -> ConfigReadError {
        ConfigReadError::FileReadError
    }
}

impl From<xdg_basedir::Error> for ConfigReadError {
    fn from(_: xdg_basedir::Error) -> ConfigReadError {
        ConfigReadError::XDGError
    }
}

pub fn parse() -> Result<Settings, ConfigReadError>
{
    let data = read_config_file();

    settings_from_string(&data?)
}

fn settings_from_string(s: &str) -> Result<Settings, ConfigReadError>
{
    let data = YamlLoader::load_from_str(s)?;
    let config = &data[0];

    let step = match config["step"].as_i64() {
        Some(x) => Some(x as i8),
        None => None
    };
    let interval = match config["interval"].as_i64() {
        Some(x) => Some(Duration::from_secs(x as u64)),
        None => None
    };

    let blacklist = read_opt_strvec(&config["blacklist"]);
    let whitelist = read_opt_strvec(&config["whitelist"]);

    Ok(Settings{step, interval, blacklist, whitelist})
}

fn read_opt_strvec(data: &Yaml) -> Option<Vec<String>>
{
    match data.as_vec() {
        Some(x) => Some(x.iter().map(|x| x.as_str().unwrap())
                        .map(|x| String::from(x))
                        .collect()),
        None => None
    }
}

fn read_config_file() -> Result<String, ConfigReadError>
{
    let mut config = xdg_basedir::get_config_home()?;
    config.push("autonice");
    config.push("config.yml");


    let mut f = File::open(&config)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;

    return Ok(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config_string() {
        let data = "step: 1
interval: 2
blacklist:
    - some
    - program
whitelist:
    - some
    - other
    - programs";

        let settings = settings_from_string(&data).unwrap();
        assert_eq!(settings.get_step(), 1);
        assert_eq!(settings.get_interval().as_secs(), 2);
        assert_eq!(settings.get_blacklist().len(), 2);
        assert_eq!(settings.get_whitelist().len(), 3);
    }
}
