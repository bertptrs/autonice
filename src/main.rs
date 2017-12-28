mod procreader;

extern crate libc;

use self::libc::{setpriority,PRIO_PROCESS};
use procreader::{read_procfs,Process};

use std::thread;
use std::time;

fn update_nice(p: &Process)
{
    if p.prog.contains("autonice") && p.nice < 19 {
        unsafe { setpriority(PRIO_PROCESS as u32, p.pid, p.nice as i32 + 1); }
    }
}

fn main()
{
    let timeout = time::Duration::from_secs(5);
    loop {
        match read_procfs() {
            Ok(x) => for p in x {
                update_nice(&p);
            },
            Err(x) => println!("Error while reading: {}", x)
        }

        thread::sleep(timeout);
    }
}
