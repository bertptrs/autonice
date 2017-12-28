extern crate libc;

use self::libc::{getuid,getpriority,PRIO_PROCESS};

use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;
use std::path::Path;

pub struct Process {
    pub pid: u32,
    pub nice: i8,
    pub prog: String
}

fn parse_entry(entry: &fs::DirEntry) -> Option<Process>
{
    let name = entry.file_name().into_string().unwrap();

    let pid = match name.parse::<u32>() {
        Ok(x) => x,
        Err(_) => { println!("Not a proc: {}", name); return None }
    };

    let mut exe_path = entry.path();
    exe_path.push("exe");

    let mut stat_path = entry.path();
    stat_path.push("stat");

    let exe = match fs::read_link(&exe_path) {
        Ok(x) => x.into_os_string(),
        Err(_) => return None,
    };

    let niceness = unsafe { getpriority(PRIO_PROCESS as u32, pid) as i8 };

    return Some(Process{pid: pid, nice: niceness, prog: String::from(exe.to_str().unwrap())});
}

pub fn read_procfs() -> io::Result<Vec<Process>>
{
    let mut result = Vec::new();
    let uid = unsafe { getuid() as u32 };

    for entry in fs::read_dir(&Path::new("/proc"))? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() && metadata.st_uid() == uid {
            match parse_entry(&entry) {
                Some(x) => result.push(x),
                None => {},
            }
        }
    }

    return Ok(result);
}
