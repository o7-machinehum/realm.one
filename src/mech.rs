use amethyst::{
    renderer::SpriteRender
};

#[allow(unused_imports)]
use log::info;
extern crate tiled;

// Collection of tools that should move to their old files
// at some point in time

pub fn get_letter(byte: u8, sprites: &Vec<SpriteRender>) -> SpriteRender {
    let mut ascii = 0;
    
    if byte >= 32 { 
        ascii = byte - 32;
    }
    // 
    // else if 97 <= byte && byte <= 122 {
    //     ascii = byte - 97
    // }
     
    sprites[ascii as usize].clone()
}
