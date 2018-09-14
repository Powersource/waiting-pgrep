extern crate clap;
extern crate psutil;

use std::vec::Vec;
use std::{thread, time::Duration};
use psutil::process::Process;

fn main() {
    loop {
        match psutil::process::all() {
            Err(why) => {
                if why.kind() == std::io::ErrorKind::NotFound {
                    // Supposedly benign error according to the documentation
                    continue;
                } else {
                    panic!("Unknown error {}", why);
                }
            },
            Ok(procs) => {
                if find_proc("nvim".to_string(), procs) {
                    // Found and printed. Quitting.
                    return;
                } else {
                    // Wait a bit and loop again
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

fn find_proc(name: String, procs: Vec<Process>) -> bool {
    let mut found = false;
    for p in procs.iter().filter(|p| p.comm == name) {
        println!("{}", p.pid);
        found = true;
    }
    return found;
}
