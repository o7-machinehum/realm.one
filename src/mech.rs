use log::info;
extern crate tiled;

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
