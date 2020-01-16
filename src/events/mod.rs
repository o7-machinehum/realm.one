extern crate tiled;

pub enum Events {
    NewMap(tiled::Map), // Change the current map 
}
