use amethyst::{
    core:: {Time},
    ecs::{System, ReadStorage, WriteStorage, Write, Join, Entities, Entity},
    renderer::SpriteRender
};

use crate::{
    network::{Pack, Cmd, Dest},
    components::{WalkAnimation, LifeformComponent},
};

pub struct ItemSyncSystem {
    delete_list : Vec::<Entity>,  // List of entities of which to delete the sync component from
}

impl ItemSyncSystem {
    pub fn new() -> Self {
        Self {
            delete_list: Vec::<Entity>::new(),
        }
    }
}

/// This system will create an item pack if an item has a sync component
impl<'s> System<'s> for ItemSyncSystem {
    type SystemData = (
        ReadStorage<'s, Item>,
        WriteStorage<'s, SyncComponent>,
        Write<'s, EventChannel<Pack>>,
        Entities<'s>,
    );

    fn run(&mut self, (entities, mut items, mut syncs, mut packs): Self::SystemData) {
        // Delete all the sync components that have already been synced.
        for sync in self.delete_list.pop() {
            syncs.remove(sync);
        }

        // Create packs and send them to the server
        for (e, item, sync) in (&entities, &items, &mut syncs).join() {
            packs.single_write(Pack::new(
                Cmd::NewItem(item.clone()), 
                Dest::All
            ));
            
            self.delete_list.push(e.clone());
        }
    }
}
