extern crate tiled;
use crate::components::PlayerInfo;

pub enum Events {
    NewMap(tiled::Map),           // Change the current map 
    CreatePlayer(Vec<PlayerInfo>),     // Insert player into the map
}
