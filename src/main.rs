#[macro_use] extern crate log;
#[macro_use] extern crate clap;

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

mod device_mapper;
use device_mapper::*;
use clap::App;

//----------------------------------------------------------------

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut dm = match device_mapper::DMIoctl::new() {
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
    }
}

//----------------------------------------------------------------

