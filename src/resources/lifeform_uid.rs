/// Lifeform UID. The server uses this to keep track of all lifeforms in the game.

pub struct LifeformUID {
    current_uid: u64,
}

impl Default for LifeformUID {
    fn default() -> Self {
        LifeformUID::new() 
    }
}

impl LifeformUID {
    pub fn new() -> Self {
        Self {
            current_uid: 0,
        }
    }
    
    /// Add a lifeform to the pool
    pub fn add(&mut self) -> u64 {
        self.current_uid += 1;
        self.current_uid 
    }
} 
