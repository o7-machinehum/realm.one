use amethyst::{
    core::{Transform},
};

struct Slot {
    taken: bool,
    location: Transform,
}

/// Inventory. This is used to keep track of items in the players inventory.
const INV_SIZE: usize = 4;
pub struct Inventory {
    slots: [Slot; INV_SIZE],
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory::new() 
    }
}

impl Inventory {
    pub fn new() -> Self {
        let mut slots: [Slot; INV_SIZE];

        for slot in &mut slots { 
           slot.taken = false;
           slot.location = Transform::default();
        }


        Self {
            slots
        }
    }
    
    /// Take a slot to place the items at random
    pub fn take(&mut self) -> Option<Transform> {
        for i in 0..self.slots.len() { 
            if !self.slots[i].taken {
                self.slots[i].taken = true;
                return Some(self.slots[i].location.clone());
            }
        }
        None
    }

    ///// Return a slot
    //pub fn rtn(&mut self, t: Transform) { 
    //    self.slots.pop()
    //}
} 
