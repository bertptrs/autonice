#[macro_use] extern crate clap;

mod procreader;
mod settings;

extern crate libc;

use self::libc::{setpriority,PRIO_PROCESS};
use procreader::{read_procfs,Process};
use std::thread;
use settings::{Settings,ConfigReadError};

fn list_contains(p: &Process, list: &Vec<String>) -> bool
{
    for entry in list {
        if p.prog.contains(entry) {
            return true;
        }
    }

    false
}

fn update_nice(p: &Process, set: &Settings)
{
    if list_contains(p, &set.get_whitelist()) && !list_contains(p, &set.get_blacklist()) {
        let new_prio = std::cmp::min(p.nice + set.get_step(), 19) as i32;

        unsafe { setpriority(PRIO_PROCESS as u32, p.pid, new_prio); }
    }
}

fn main()
{
    let mut set = Settings::from_cli();
    match Settings::from_config() {
        Ok(mut c) => { c.amend(set); set = c},
        Err(ConfigReadError::InvalidYamlError) => panic!("Invalid YAML config file"),
        Err(_) => {},
    }
    loop {
        match read_procfs() {
            Ok(x) => for p in x {
                update_nice(&p, &set);
            },
            Err(x) => println!("Error while reading: {}", x)
        }

        thread::sleep(set.get_interval());
    }
}
