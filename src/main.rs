// FIXME: remove
#![allow(unused)]

#[macro_use] extern crate bitflags;
#[macro_use] extern crate clap;
#[macro_use] extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate nix;

extern crate libc;

// Low level modules:
//
// Config: There's a nice combinator library called 'nom' that we can use to
// implement the config file parser.
//
// log: Need a generic logging facility (I think there's a standard trait for this).
//
// dm-trait: Abstract the dm interface.  We need this for testing.
// Must be able to inject errors.
//
// dm-ioctl: I think we'll have to use unsafe Rust to do this, so
// leave until more experienced.  Building up the command buffers
// should just be a question of building a suitable Vec<u8>, so
// perhaps not that tricky.

extern crate regex;

//mod config;
mod device_mapper;
mod types;

use clap::App;
use device_mapper::low_level::*;
use device_mapper::high_level::*;
use device_mapper::linear_target::*;
use regex::Regex;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

// FIXME: stop using unwrap everywhere.
// FIXME: implement disk units

//----------------------------------------------------------------

struct TestConfig {
    metadata_dev: PathBuf,
    data_dev: PathBuf
}

fn scenario_basic_overwrite_linear(cfg: &TestConfig, dm: &mut DMInterface) -> io::Result<()> {
    // Build up the table string
    let mut table = Table::new();

    table.push(
        LinearTarget {
            dev: cfg.data_dev.clone(),
            begin: 0,
            end: 2097152            // 1 gig
        }).push(
        LinearTarget {
            dev: cfg.data_dev.clone(),
            begin: 2097152,
            end: 2097152 * 2
        });

    let id = DMIdentity::Name("foo");
    try!(dm.create(&id));
    try!(dm.load(&id, &table.to_dm_targets()));
    try!(dm.resume(&id));

    let output = try!(Command::new("ls")
                      .arg("-la")
                      .output());

    println!("process output = '{:?}'", output.stdout);
    
    try!(dm.remove(&id));
    
    Ok(())
}

//----------------------------------------------------------------

fn contains_text(s: &str) -> bool {
    !s.chars().all(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n')
}

fn parse_table(str: &String) -> io::Result<Vec<DmTarget>> {
    let mut r = Vec::with_capacity(8);
    let re = Regex::new(r"^([0-9]+)\s+([0-9]+)\s+([a-zA-Z]+)\s+(.*)$").unwrap();

    for l in str.split("\\n").filter(|&s| contains_text(s)) {
        let bits = match re.captures(l) {
            Some(n) => n,
            None => return Err(Error::new(ErrorKind::Other,
                                            "couldn't parse target line"))
        };
                
        let begin = match bits[1].parse::<u64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::new(ErrorKind::Other,
                                            "couldn't parse sector begin"))
        };
        
        let end = match bits[2].parse::<u64>() {
            Ok(n) => n,
            Err(_) => return Err(Error::new(ErrorKind::Other,
                                            "couldn't parse sector end"))
        };
        
        let mut ttype = String::new();
        ttype.push_str(&bits[3]);
        let mut args = String::new();
        args.push_str(&bits[4]);
        let t = DmTarget { target_type: ttype,
                           sector_begin: begin,
                           sector_end: end,
                           ctr_args: args };
        println!("pushing {:?}", t);
        r.push(t)
    }

    Ok(r)
}

//----------------------------------------------------------------

fn dmsetup() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut dm = match device_mapper::low_level::DMIoctl::new() {
        Ok(dm) => dm,
        Err(e) => {
            println!("Couldn't create dm interface: {:?}", e);
            return;
        }
    };

    if let Some(matches) = matches.subcommand_matches("create") {
        let name = matches.value_of("NAME").unwrap();
        let id = DMIdentity::Name(name);
        dm.create(&id).unwrap();

    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let name = matches.value_of("NAME").unwrap();
        let id = DMIdentity::Name(name);
        dm.remove(&id).unwrap();

    } else if let Some(_) = matches.subcommand_matches("remove_all") {
        dm.remove_all().unwrap();

    } else if let Some(_) = matches.subcommand_matches("ls") {
        let ds = dm.list_devices().unwrap();

        for dev in ds {
            println!("{}\t({}:{})", dev.name, dev.major, dev.minor);
        }

    } else if let Some(matches) = matches.subcommand_matches("suspend") {
        let name = matches.value_of("NAME").unwrap();
        let id = DMIdentity::Name(name);
        dm.suspend(&id).unwrap();

    } else if let Some(matches) = matches.subcommand_matches("resume") {
        let name = matches.value_of("NAME").unwrap();
        let id = DMIdentity::Name(name);
        dm.resume(&id).unwrap();

    } else if let Some(matches) = matches.subcommand_matches("load") {
        let name = matches.value_of("NAME").unwrap();
        let table = matches.value_of("table").unwrap();
        let id = DMIdentity::Name(name);

        let input = String::from(table);
        println!("name = {}", name);
        dm.load(&id, &parse_table(&input).unwrap()).unwrap();
    }
}

//----------------------------------------------------------------

fn main() {
    env_logger::init().unwrap();
    info!("Logger initialised");
    
    let cfg = TestConfig {
        metadata_dev: PathBuf::from("/dev/sda"),
        data_dev: PathBuf::from("/dev/sdb")
    };
    

    let mut dm = match device_mapper::low_level::DMIoctl::new() {
        Ok(dm) => dm,
        Err(e) => {
            println!("Couldn't create dm interface: {:?}", e);
            return;
        }
    };
    
    scenario_basic_overwrite_linear(&cfg, &mut dm).unwrap();
}

//----------------------------------------------------------------

