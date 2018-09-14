#[macro_use]
extern crate clap;
extern crate psutil;

use std::vec::Vec;
use std::{thread, time::Duration};
use psutil::process::Process;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Waiting pgrep")
        .version(crate_version!())
        .arg(Arg::with_name("name")
            .help("The program name to look for")
            .required(true)
            .index(1))
        .get_matches();

    let name = matches.value_of("name").unwrap().to_string();

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
                if find_proc(&name, procs) {
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

fn find_proc(name: &String, procs: Vec<Process>) -> bool {
    let mut found = false;
    for p in procs.iter().filter(|p| &p.comm == name) {
        println!("{}", p.pid);
        found = true;
    }
    return found;
}
