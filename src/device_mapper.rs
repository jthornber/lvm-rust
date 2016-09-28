// FIXME: remove
#![allow(unused)]

use std::fs::File;
// use std::io::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std;

//----------------------------------------------------------------

const DM_NAME_LEN: usize = 128;
const DM_UUID_LEN: usize = 129;

#[repr(C, packed)]
struct IoctlHeader {
    version: [u32; 3],
    data_size: u32,
    data_start: u32,
    target_count: u32,
    open_count: i32,
    flags: u32,
    event_nr: u32,
    padding: u32,
    dev: u64,
    name: [u8; DM_NAME_LEN],
    uuid: [u8; DM_UUID_LEN],

    // padding
    data: [u8; 7]
}

//----------------------------------------------------------------

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

struct IoctlBuffer {
    buffer: Vec<u8>
}

impl IoctlBuffer {
    fn new(payload_size: usize) -> IoctlBuffer {
        IoctlBuffer { buffer: Vec::with_capacity(payload_size + std::mem::size_of::<IoctlHeader>()) }
    }


}

//--------------------------------

// use as_raw_fd method which returns the c_int descriptor

pub struct DMIoctl {
    control_file: File
}

impl DMIoctl {
    fn new_specifying_control<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<DMIoctl> {
        Ok(DMIoctl { control_file: try!(File::open(&path)) })
    }

    pub fn new() -> io::Result<DMIoctl> {
        let control_path = Path::new("/dev/mapper/control");
        Self::new_specifying_control(control_path)
    }
}

fn not_implemented<T>() -> io::Result<T> {
    Err(Error::new(ErrorKind::Other, "not implemented"))
}

impl DMInterface for DMIoctl {
    fn version(&self) -> io::Result<(u32, u32, u32)> {
        not_implemented()
    }

    fn remove_all(&self) -> io::Result<()> {
        not_implemented()
    }

    fn list_devices(&self) -> io::Result<Vec<DeviceInfo>> {
        not_implemented()
    }

    fn create(&self, n: &NameOrDevId) -> io::Result<()> {
        not_implemented()
    }

    fn remove(&self, n: &NameOrDevId) -> io::Result<()> {
        not_implemented()
    }

    fn suspend(&self, n: &NameOrDevId) -> io::Result<()> {
        not_implemented()
    }

    fn resume(&self, n: &NameOrDevId) -> io::Result<()> {
        not_implemented()
    }

    fn clear(&self, n: &NameOrDevId) -> io::Result<()> {
        not_implemented()
    }

    fn load(&self, n: &NameOrDevId, targets: &Vec<String>) -> io::Result<()> {
        not_implemented()
    }

    fn status(&self, n: &NameOrDevId) -> io::Result<Vec<String>> {
        not_implemented()
    }

    fn table(&self, n: &NameOrDevId) -> io::Result<Vec<String>> {
        not_implemented()
    }

    fn info(&self, n: &NameOrDevId) -> io::Result<Vec<String>> {
        not_implemented()
    }

    fn message(&self, n: &NameOrDevId, msg: &str, sector: u64) -> io::Result<()> {
        not_implemented()
    }
}

//----------------------------------------------------------------
