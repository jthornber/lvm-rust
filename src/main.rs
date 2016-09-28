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

// FFI:
// use #[repr(C, packed)]

use std::fs::File;
// use std::io::prelude::*;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;

pub struct DeviceInfo {
    pub major: u32,
    pub minor: u32,
    pub name: String
}

pub enum NameOrDevId<'a> {
    Name(&'a str),
    DevId(u32, u32)
}

pub enum DMErr {

}

pub trait DMInterface {
    fn version(&self) -> io::Result<(u32, u32, u32)>;
    fn remove_all(&self) -> io::Result<()>;
    fn list_devices(&self) -> io::Result<Vec<DeviceInfo>>;
    fn create(&self, n: &NameOrDevId) -> io::Result<()>;
    fn remove(&self, n: &NameOrDevId) -> io::Result<()>;
    fn suspend(&self, n: &NameOrDevId) -> io::Result<()>;
    fn resume(&self, n: &NameOrDevId) -> io::Result<()>;
    fn clear(&self, n: &NameOrDevId) -> io::Result<()>;
    fn load(&self, n: &NameOrDevId, targets: &Vec<String>) -> io::Result<()>;
    fn status(&self, n: &NameOrDevId) -> io::Result<Vec<String>>;
    fn table(&self, n: &NameOrDevId) -> io::Result<Vec<String>>;
    fn info(&self, n: &NameOrDevId) -> io::Result<Vec<String>>;
    fn message(&self, n: &NameOrDevId, msg: &str, sector: u64) -> io::Result<()>;
}

//----------------------------------------------------------------

// use as_raw_fd method which returns the c_int descriptor

pub struct DMIoctl {
    control_file: File
}

impl DMIoctl {
    fn new_specifying_control<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<DMIoctl> {
        Ok(DMIoctl { control_file: try!(File::open(&path)) })
    }
        
    fn new() -> io::Result<DMIoctl> {
        let control_path = Path::new("/dev/mapper/control");
        Self::new_specifying_control(control_path)
    }
}

//----------------------------------------------------------------

fn main() {
    match DMIoctl::new() {
        Ok(dm) => println!("Opened dm control file"),
        Err(e) => println!("Couldn't create dm interface: {:?}", e)
    }       
}
