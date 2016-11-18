use std::fmt::Debug;

use device_mapper::low_level::*;
use types::*;

pub trait Target: Debug {
    fn sectors(&self) -> Sector;
    fn to_dm_target(&self, offset: Sector) -> DmTarget;
}

#[derive(Debug)]
pub struct Table<'a> {
    targets: Vec<Box<Target + 'a>>
}

impl<'a> Table<'a> {
    pub fn new() -> Table<'a> {
        Table { targets: vec![] }
    }
    
    pub fn push<T: Target + 'a>(&mut self, t: T) -> &mut Table<'a> {
        self.targets.push(Box::new(t));
        self
    }

    pub fn to_dm_targets(&self) -> Vec<DmTarget> {
        let mut r = vec![];
        let mut offset = 0;
        for t in &self.targets {
            r.push(t.to_dm_target(offset));
            offset += t.sectors();
        }

        r
    }
}
