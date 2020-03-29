use crate::components::{Skins, Outfit, get_outfit, outfit_from_str};

extern crate tiled;

/// This should be moved somewhere else in the future
pub fn get_string(keyword: String, tile: &tiled::Properties) -> Option<String> {
    match tile.get(&keyword) {
            Some(value) => match value {
                tiled::PropertyValue::StringValue(val) => return Some(val.to_string()), 
                _ => None
            },
            None => None
    }
}

pub fn get_float(keyword: String, tile: &tiled::Properties) -> Option<f32> {
    match tile.get(&keyword) {
            Some(value) => match value {
                tiled::PropertyValue::FloatValue(val) => return Some(*val), 
                _ => None
            },
            None => None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    name: String,
    x: f32,          
    y: f32, 
    skin: Outfit,
    hp: f32,
}

impl Monster {
    pub fn new(prop: tiled::Properties, pos: (u32, u32)) -> Self {
        Self {
            name: get_string("Name".to_string(), &prop).unwrap(),
            x: (pos.0 as f32) * 8.0,
            y: (pos.1 as f32) * 8.0, 
            skin: outfit_from_str(get_string("Skin".to_string(), &prop).unwrap()),
            hp: get_float("HP".to_string(), &prop).unwrap(),
        }
    }
}
