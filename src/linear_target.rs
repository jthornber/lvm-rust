use std::path::{Path, PathBuf};

use types::*;
use device_mapper::*;
use device_mapper_high_level::*;

#[derive(Debug)]
pub struct LinearTarget {
    pub dev: PathBuf,               // FIXME: why not a plain Path?
    pub begin: Sector,
    pub end: Sector
}

impl Target for LinearTarget {
    fn sectors(&self) -> Sector {
        self.end - self.begin
    }

    fn to_dm_target(&self, offset: Sector) -> DmTarget {
        DmTarget {
            target_type: String::from("linear"),
            sector_begin: offset,
            sector_end: offset + self.end,
            ctr_args: format!("{} {}", self.dev.display(), self.begin)
        }
    }
}
