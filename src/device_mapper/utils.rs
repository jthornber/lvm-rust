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

#[cfg(test)]
mod tests {
    fn __parse_test(t: (&Option<Vec<u32>>, &[u8])) {
        let mut ptr = &t.1[..];
        assert_eq!(super::parse_majors(&mut ptr), *t.0);
    }

    #[test]
    fn test_parse_majors() {
        let empty = Some(vec![]);

        __parse_test((&empty, b""));
        __parse_test((&empty, b"flippity"));

        __parse_test((&empty,
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty"#));

        __parse_test((&empty,
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty
128 device-mapper"#));

        __parse_test((&Some(vec![128u32]),
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty

Block devices:
128 device-mapper"#));

        __parse_test((&Some(vec![28u32]),
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty

Block devices:
 28 device-mapper"#));

        __parse_test((&Some(vec![28u32]),
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty

Block devices:
 28 device-mapper"#));

        __parse_test((&Some(vec![28u32, 128]),
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty

Block devices:
  1 foo
 28 device-mapper
111 bar
128 device-mapper
199 hux"#));

        __parse_test((&None,
                      br#"Character devices:
  1 mem
  4 /dev/vc/0
  4 tty

Block devices:
  1 foo
 28 device-mapper
111 bar
1o8 device-mapper
199 hux"#));
    }
}

/// Read and parse /proce/devices to ascertain the major numbers used
/// by DM.
pub fn dm_majors() -> io::Result<Vec<u32>> {
    let f = try!(File::open("/proc/devices"));
    let mut reader = io::BufReader::new(f);
    match parse_majors(&mut reader) {
        Some(majors) => Ok(majors),
        None => Err(Error::new(ErrorKind::Other, "parse error"))
    }
}
