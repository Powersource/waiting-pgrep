extern crate clap;
extern crate psutil;

use std::vec::Vec;
use psutil::process::Process;

fn main() {
    println!("Hello, world!");

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
                return find_proc("code-oss".to_string(), procs);
            }
        }
    }
}

fn find_proc(name: String, procs: Vec<Process>) {
    for p in procs.iter().filter(|p| p.comm == name) {
        println!("{}", p.pid);
    }
}
