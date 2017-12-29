extern crate libc;
extern crate nix;

use self::libc::{setpriority,PRIO_PROCESS};
use self::nix::sys::signal;
use settings::Settings;
use std::cmp::min;
use std::thread;
use procreader::{read_procfs,Process};

static mut SHOULD_RUN: bool = false;

fn list_contains(p: &Process, list: &[String]) -> bool
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
        let new_prio = min(p.nice + set.get_step(), 19) as i32;

        unsafe { setpriority(PRIO_PROCESS as u32, p.pid, new_prio); }
    }
}

pub fn autonice(set: &Settings)
{
    let action = signal::SigAction::new(signal::SigHandler::Handler(handle_sigint),
                                        signal::SaFlags::empty(),
                                        signal::SigSet::empty());
    unsafe {
        SHOULD_RUN = true;
        signal::sigaction(signal::SIGINT, &action).expect("Couldn't register signal handler");
    }
    while unsafe {SHOULD_RUN} {
        match read_procfs() {
            Ok(x) => for p in x {
                update_nice(&p, &set);
            },
            Err(x) => println!("Error while reading: {}", x)
        }

        thread::sleep(set.get_interval());
    }
}

extern fn handle_sigint(_: i32)
{
    println!("Received SIGINT, shutting down.");
    unsafe {SHOULD_RUN = false;}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_contains()
    {
        let p = Process{nice: 0, prog: "make".to_owned(), pid: 1};
        assert!(list_contains(&p, &vec![String::from("not right"), String::from("next one"), String::from("make")]));
    }

    #[test]
    #[should_panic]
    fn test_list_not_contains()
    {
        let p = Process{nice: 0, prog: "make".to_owned(), pid: 1};
        assert!(list_contains(&p, &vec![String::from("not right"), String::from("nope"), String::from("not it")]));
    }
}
