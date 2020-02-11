use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Skins {
    Nude,
    Male,
    Female,
    Skeleton,
    Slime,
    Bat,
    Ghost,
    Spider,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Outfit {
    pub n: usize,      
    pub e: usize, 
    pub s: usize,
    pub w: usize, 
}

pub fn get_outfit(skin: &Skins) -> Outfit {
    match skin {
        Skins::Nude     => Outfit { n: 312, e: 300, s: 276, w: 288 },
        Skins::Male     => Outfit { n: 315, e: 303, s: 279, w: 291 },
        Skins::Female   => Outfit { n: 319, e: 307, s: 283, w: 295 },
        Skins::Skeleton => Outfit { n: 321, e: 309, s: 285, w: 297 },
        Skins::Slime    => Outfit { n: 360, e: 349, s: 324, w: 336 },
        Skins::Bat      => Outfit { n: 363, e: 352, s: 327, w: 339 },
        Skins::Ghost    => Outfit { n: 366, e: 355, s: 330, w: 342},
        Skins::Spider   => Outfit { n: 369, e: 358, s: 233, w: 245 },
    }
}
