use amethyst::{
    renderer::SpriteRender
};

use log::info;
extern crate tiled;

// Collection of tools that should move to their old files
// at some point in time

pub fn colision(tile: &Option<tiled::Properties>) -> bool{
    match tile {
        None => return false,
        Some(i) => match i.get("Collision") {
            Some(value) => match value {
                tiled::PropertyValue::BoolValue(val) => return *val, 
                tiled::PropertyValue::FloatValue(val) => info!("{}", val), 
                tiled::PropertyValue::IntValue(val) => info!("{}", val), 
                tiled::PropertyValue::ColorValue(val) => info!("{}", val), 
                tiled::PropertyValue::StringValue(val) => info!("{}", val), 
            },
            None => return false,
        },
    }
    return true
}

pub fn get_letter(byte: u8, sprites: &Vec<SpriteRender>) -> SpriteRender {
    let mut ascii = 0;
    
    if 65 <= byte && byte <= 90 { 
        ascii = byte - 65;
    }
    
    else if 97 <= byte && byte <= 122 {
        ascii = byte - 97
    }
     
    sprites[ascii as usize].clone()
}
