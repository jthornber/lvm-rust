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

//----------------------------------------------------------------

fn main() {
    match device_mapper::DMIoctl::new() {
        Ok(dm) => println!("Opened dm control file"),
        Err(e) => println!("Couldn't create dm interface: {:?}", e)
    }       
}

//----------------------------------------------------------------
