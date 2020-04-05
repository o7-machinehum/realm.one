use crate::network::Pack;

/// IO resource, buff for inputs and outputs
pub struct IO {
    pub i: Vec<Pack>,
    pub o: Vec<Pack>,
}

impl IO {
    pub fn new() -> Self {
        Self {
            i: Vec::<Pack>::new(),
            o: Vec::<Pack>::new(),
        }
    }
}

impl Default for IO {
    fn default() -> Self {
        Self {
            i: Vec::<Pack>::new(),
            o: Vec::<Pack>::new(),
        }
    }
}
