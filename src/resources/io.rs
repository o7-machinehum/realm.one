use crate::network::{Pack, Cmdb};
use std::collections::HashMap;

pub struct Packs {
    packs: HashMap<Cmdb, Vec<Pack>>,
}

impl Packs {
    pub fn new() -> Self {
        Self {
            packs: HashMap::<Cmdb, Vec<Pack>>::new(),
        }
    }

    pub fn put(&mut self, cmd: Cmdb, pack: Pack) {
        match self.packs.get_mut(&cmd) {
            Some(pk) => {pk.push(pack); None},
            None => self.packs.insert(cmd, Vec::<Pack>::new()),
        };
    }

    pub fn get(&mut self, cmd: Cmdb) -> Option<Vec<Pack>> {
        self.packs.remove(&cmd)
    }
}

pub struct NetInputs {
   pub packs: Packs,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            packs: Packs::new(),
        }
    }   
}

pub struct NetOutputs {
   pub packs: Packs,
}

impl Outputs {
    pub fn new() -> Self {
        Self {
            packs: Packs::new(),
        }
    }   
}

impl Default for Outputs {
    fn default() -> Self {
        Outputs::new()
    }
}

impl Default for Inputs {
    fn default() -> Self {
        Outputs::new()
    }
}
