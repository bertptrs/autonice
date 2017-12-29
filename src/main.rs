#[macro_use] extern crate clap;

mod procreader;
mod settings;
mod nicer;

use settings::{Settings,ConfigReadError};

fn main()
{
    let mut set = Settings::from_cli();
    match Settings::from_config() {
        Ok(mut c) => { c.amend(set); set = c},
        Err(ConfigReadError::InvalidYamlError) => panic!("Invalid YAML config file"),
        Err(_) => {},
    }

    nicer::autonice(&set);
}
