use amethyst::{
    core::{Transform},
};

/// Inventory. This is used to keep track of items in the players inventory.
const inv_size: usize = 4;
pub struct Inventory {
    slots: [bool; inv_size],
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory::new() 
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            slots: [false, false, false, false],
        }
    }
    
    /// Take a slot to place the items at random
    pub fn take(&mut self) -> Option<Transform> {
        for i in 0..self.slots.len() { 
            if !self.slots[i] {
                self.slots[i] = true;
                return Some(Transform::default());
            }
        }
        None
    }

    ///// Return a slot
    //pub fn rtn(&mut self, t: Transform) { 
    //    self.slots.pop()
    //}
} 
