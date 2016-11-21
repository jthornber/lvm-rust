use std;
use std::io;
use std::io::*;
use std::fs::File;

// Various useful bits of code picked up from Andy Grover's codebase.

/// Parses input with the same format as /proc/devices.
fn parse_majors(reader: &mut std::io::BufRead) -> Option<Vec<u32>> {
    let mut majors = Vec::<u32>::with_capacity(4);

    for line in reader.lines()
        .filter_map(|x| x.ok())
        .skip_while(|x| x != "Block devices:")
        .skip(1) {
            let fields: Vec<_> = line.split_whitespace().collect();

            if fields.len() == 2 &&
                fields[1] == "device-mapper" {
                    match fields[0].parse::<u32>() {
                        Ok(n) => majors.push(n),
                        Err(_) => {
                            // FIXME: add log message
                            return None;
                        }
                    }
                }
        }

    Some(majors)
}

// FIXME: write test

/// Major numbers used by DM.
pub fn dm_majors() -> io::Result<Vec<u32>> {
    let f = try!(File::open("/proc/devices"));
    let mut reader = io::BufReader::new(f);
    match parse_majors(&mut reader) {
        Some(majors) => Ok(majors),
        None => Err(Error::new(ErrorKind::Other, "parse error"))
    }
}
