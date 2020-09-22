
use amethyst::{
    core::{Transform},
};

struct Slot {
    taken: bool,
    location: Transform,
}

impl Slot {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            taken: false,
            location: Transform::default(),
        }
    }
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
        unsafe { 
            #[allow(deprecated)]
            let mut slots: [Slot; INV_SIZE] = std::mem::uninitialized();
            
            let mut loc = Transform::default();
            loc.set_translation_xyz(776.0, 680.0, 1.0); 
            for slot in &mut slots {
                slot.taken = false;
                loc.move_right(16.0);
                slot.location = loc.clone();
            }

            Self {
                slots
            }
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
} 
