// FIXME: remove
#![allow(unused)]

extern crate libc;

use std::fs::File;
// use std::io::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std;

//----------------------------------------------------------------

/// * Utilities

/// Zero a u8 iterator
fn zero<'a, I: Iterator<Item=&'a mut u8>>(it: I) {
    for d in it {
        *d = 0
    }
}

/// Copy chars to a u8 iterator, then pad the rest with zeroes
fn copy_and_pad<'a, ISrc, IDest>(dest: IDest, src: ISrc)
    where IDest: Iterator<Item=&'a mut u8>,
          ISrc: Iterator<Item=char>
{
    for (d, s) in dest.zip(src.chain(std::iter::repeat('\0'))) {
        *d = s as u8
    }
}

// FIXME: there must be a version of this in the std library
fn from_c_str<ISrc: Iterator<Item=u8>>(src: ISrc) -> String {
    let mut r = String::new();

    for c in src {
        if c == 0 { break; }
        r.push(c as char)
    }

    r
}

//----------------------------------------------------------------

const DM_NAME_LEN: usize = 128;
const DM_UUID_LEN: usize = 129;

const DM_VERSION_MAJOR: u32 = 4;
const DM_VERSION_MINOR: u32 = 27;
const DM_VERSION_PATCH: u32 = 0;

//----------------------------------------------------------------

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

enum DmFlags {
    DmReadOnlyBit = 0,
    DmSuspendBit = 1,
    DmPersistentDevBit = 3,
    DmStatusTableBit = 4,
    DmActivePresentBit = 5,
    DmInactivePresentBit = 6,
    DmBufferFullBit = 8,
    DmSkipBDGetBit = 9,
    DmSkipLockFSBit = 10,
    DmNoFlushBit = 11,
    DmQueryInactiveTableBit = 12,
    DmUeventGeneratedBit = 13,
    DmUuidBit = 14,
    DmSecureDataBit = 15,
    DmDataOutBit = 16,
    DmDeferredRemoveBit = 17
}

impl IoctlHeader {
    fn new() -> IoctlHeader {
        IoctlHeader {
            version: [0, 0, 0],
            data_size: 0,
            data_start: 0,
            target_count: 0,
            open_count: 0,
            flags: 0,
            event_nr: 0,
            padding: 0,
            dev: 0,
            name: [0; DM_NAME_LEN],
            uuid: [0; DM_UUID_LEN],
            data: [0; 7]
        }
    }

    /// Returns true on success.  Can only fail if the provided
    /// identity is too long.
    fn set_identity(&mut self, id: &DMIdentity) -> bool {
        match *id {
            // FIXME: factor out common code
            DMIdentity::UUID(str) => {
                if str.len() >= DM_UUID_LEN {
                    return false;
                }

                copy_and_pad(self.uuid.iter_mut(), str.chars());
                zero(self.name.iter_mut());
            },

            DMIdentity::Name(str) => {
                println!("setting name to {}", &str);

                if str.len() >= DM_NAME_LEN {
                    return false;
                }

                copy_and_pad(self.name.iter_mut(), str.chars());
                zero(self.uuid.iter_mut());

                println!("{} {} {} {}", self.name[0], self.name[1], self.name[2], self.name[3]);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::IoctlHeader;

    #[test]
    fn test_set_identity() {
        let mut h = IoctlHeader::new();
        let mut too_long = String::new();
        for i in 0..1024 {
            too_long.push('a')
        }

        assert_eq!(h.set_identity(&DMIdentity::Name(&too_long)), false);

        // setting the name
        assert_eq!(h.set_identity(&DMIdentity::Name("foo")), true);
        assert_eq!(h.name[0], 'f' as u8);
        assert_eq!(h.uuid[0], 0);

        // setting the uuid
        assert_eq!(h.set_identity(&DMIdentity::UUID("asldfkj")), true);
        assert_eq!(h.uuid[0], 'a' as u8);
        assert_eq!(h.name[0], 0);

        // shortening name works
        h.set_identity(&DMIdentity::Name("a-long-name"));
        h.set_identity(&DMIdentity::Name("foo"));
        assert_eq!(h.name[3], 0);

        // shortening name works
        h.set_identity(&DMIdentity::UUID("a-long-name"));
        h.set_identity(&DMIdentity::UUID("foo"));
        assert_eq!(h.uuid[3], 0);
    }

    fn prep_cstr<T: IntoIterator>(src: T) -> Vec<u8> {
        let mut r = Vec::<u8>::new();
        for c in src {
            r.push(c as u8)
        }
        r.push(0);

        r
    }

    #[test]
    fn test_from_c_str() {
        let mut cs = prep_cstr("foo");
    }
}

//----------------------------------------------------------------

pub struct DeviceInfo {
    pub major: u32,
    pub minor: u32,
    pub name: String
}

#[derive(Debug)]
pub enum DMIdentity<'a> {
    Name(&'a str),
    UUID(&'a str)
}

pub enum DMErr {

}

pub trait DMInterface {
    fn version(&mut self) -> io::Result<(u32, u32, u32)>;
    fn remove_all(&mut self) -> io::Result<()>;
    fn list_devices(&mut self) -> io::Result<Vec<DeviceInfo>>;
    fn create(&mut self, n: &DMIdentity) -> io::Result<()>;
    fn remove(&mut self, n: &DMIdentity) -> io::Result<()>;
    fn suspend(&mut self, n: &DMIdentity) -> io::Result<()>;
    fn resume(&mut self, n: &DMIdentity) -> io::Result<()>;
    fn clear(&mut self, n: &DMIdentity) -> io::Result<()>;
    fn load(&mut self, n: &DMIdentity, targets: &Vec<String>) -> io::Result<()>;
    fn status(&mut self, n: &DMIdentity) -> io::Result<Vec<String>>;
    fn table(&mut self, n: &DMIdentity) -> io::Result<Vec<String>>;
    fn info(&mut self, n: &DMIdentity) -> io::Result<Vec<String>>;
    fn message(&mut self, n: &DMIdentity, msg: &str, sector: u64) -> io::Result<()>;
}

//----------------------------------------------------------------

struct IoctlBuffer {
    buffer: Vec<u8>
}

impl IoctlBuffer {
    fn new(payload_size: usize) -> IoctlBuffer {
        let total_size = payload_size + std::mem::size_of::<IoctlHeader>();
        let mut buf = IoctlBuffer { buffer: Vec::with_capacity(total_size) };

        let header: &mut IoctlHeader = unsafe { &mut *buf.get_header_mut() };

        header.version[0] = DM_VERSION_MAJOR;
        header.version[1] = DM_VERSION_MINOR;
        header.version[2] = DM_VERSION_PATCH;
        header.data_size = total_size as u32;
        header.data_start = std::mem::size_of::<IoctlHeader>() as u32;

        buf
    }

    fn total_size(&self) -> usize {
        let header: &IoctlHeader = unsafe { &*self.get_header() };
        header.data_size as usize
    }

    fn expand(&self) -> IoctlBuffer {
        let size = self.total_size();
        let mut r = IoctlBuffer::new(2 * size);
        r.buffer[0..size].copy_from_slice(&self.buffer[0..size]);

        r
    }

    unsafe fn get_raw(&self) -> *const u8 {
        (&self.buffer).as_ptr()
    }

    unsafe fn get_raw_mut(&mut self) -> *mut u8 {
        (&mut self.buffer).as_mut_ptr()
    }

    unsafe fn get_header(&self) -> *const IoctlHeader {
        self.get_raw() as *const IoctlHeader
    }

    unsafe fn get_header_mut(&mut self) -> *mut IoctlHeader {
        self.get_raw_mut() as *mut IoctlHeader
    }

    unsafe fn get_payload(&mut self) -> *mut u8 {
        (&mut self.buffer[std::mem::size_of::<IoctlHeader>()..]).as_mut_ptr()
    }
}

//--------------------------------

#[repr(C, packed)]
struct DmNameList {
    dev: u64,
    next: u32,
    name: [u8; DM_NAME_LEN]
}

//--------------------------------

#[derive(Debug, Copy, Clone)]
enum IoctlCode {
    DmVersion = 3241737472,
    DmRemoveAll = 3241737473,
    DmListDevices = 3241737474,
    DmDevCreate = 3241737475,
    DmDevRemove = 3241737476,
    DmDevRename = 3241737477,
    DmDevSuspend = 3241737478,
    DmDevStatus = 3241737479,
    DmDevWait = 3241737480,
    DmTableLoad = 3241737481,
    DmTableClear = 3241737482,
    DmTableDeps = 3241737483,
    DmTableStatus = 3241737484,
    DmListVersions = 3241737485,
    DmTargetMsg = 3241737486,
    DmDevSetGeometry = 3241737487,
}

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

    // We may be forced to resize the buffer, so we take ownership of
    // the buffer, and return a possibly new one.
    fn exec(&mut self, cmd: IoctlCode, buf: IoctlBuffer) -> io::Result<IoctlBuffer> {
        unsafe {
            let r = libc::ioctl(self.control_file.as_raw_fd(), cmd as u64, buf.get_raw());
            if r == 0 {
                let header: &IoctlHeader = &*buf.get_header();
                if header.flags & (1 << (DmFlags::DmBufferFullBit as usize)) != 0 {
                    self.exec(cmd, buf.expand())
                } else {
                    Ok(buf)
                }
            } else {
                Err(Error::new(ErrorKind::Other,
                               format!("{}", Error::last_os_error())))
            }
        }
    }

    fn exec_void(&mut self, cmd: IoctlCode) -> io::Result<()> {
        try!(self.exec(cmd, IoctlBuffer::new(0)));
        Ok(())
    }

    fn exec_void_with_name(&mut self, cmd: IoctlCode, id: &DMIdentity) -> io::Result<()> {
        let mut buf = IoctlBuffer::new(0);
        let header: &mut IoctlHeader = unsafe { &mut *buf.get_header_mut() };
        header.set_identity(id);
        try!(self.exec(cmd, buf));
        Ok(())
    }
}

fn not_implemented<T>() -> io::Result<T> {
    Err(Error::new(ErrorKind::Other, "not implemented"))
}

impl DMInterface for DMIoctl {
    fn version(&mut self) -> io::Result<(u32, u32, u32)> {
        let buf = try!(self.exec(IoctlCode::DmVersion, IoctlBuffer::new(0)));
        let header: &IoctlHeader = unsafe { &*buf.get_header() };
        Ok((header.version[0], header.version[1], header.version[2]))
    }

    fn remove_all(&mut self) -> io::Result<()> {
        self.exec_void(IoctlCode::DmRemoveAll)
    }

    fn list_devices(&mut self) -> io::Result<Vec<DeviceInfo>> {
        let buf = try!(self.exec(IoctlCode::DmListDevices,
                                 IoctlBuffer::new(8192)));
        let mut devs = Vec::<DeviceInfo>::with_capacity(8);
        // unsafe {
        //     loop {
        //         let nl = buf.get_raw() as *const DmNameList;

        //         if nl.dev == 0 { break; }

        //         devs.push(DeviceInfo { major: 0,
        //                                minor: 0,
        //                                name: n)
        // }

        Ok(devs)
    }

    fn create(&mut self, n: &DMIdentity) -> io::Result<()> {
        self.exec_void_with_name(IoctlCode::DmDevCreate, n)
    }

    fn remove(&mut self, n: &DMIdentity) -> io::Result<()> {
        self.exec_void_with_name(IoctlCode::DmDevRemove, n)
    }

    fn suspend(&mut self, n: &DMIdentity) -> io::Result<()> {
        self.exec_void_with_name(IoctlCode::DmDevSuspend, n)
    }

    fn resume(&mut self, n: &DMIdentity) -> io::Result<()> {
        // FIXME: set the resume flag
        self.exec_void_with_name(IoctlCode::DmDevSuspend, n)
    }

    fn clear(&mut self, n: &DMIdentity) -> io::Result<()> {
        self.exec_void_with_name(IoctlCode::DmTableClear, n)
    }

    fn load(&mut self, n: &DMIdentity, targets: &Vec<String>) -> io::Result<()> {
        not_implemented()
    }

    fn status(&mut self, n: &DMIdentity) -> io::Result<Vec<String>> {
        not_implemented()
    }

    fn table(&mut self, n: &DMIdentity) -> io::Result<Vec<String>> {
        not_implemented()
    }

    fn info(&mut self, n: &DMIdentity) -> io::Result<Vec<String>> {
        not_implemented()
    }

    fn message(&mut self, n: &DMIdentity, msg: &str, sector: u64) -> io::Result<()> {
        not_implemented()
    }
}

//----------------------------------------------------------------
