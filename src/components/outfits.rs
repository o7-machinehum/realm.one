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
        Skins::Nude     => Outfit {s: 277, w: 289, e: 301, n: 313 },
        Skins::Male     => Outfit {s: 280, w: 292, e: 304, n: 316 },
        Skins::Female   => Outfit {s: 283, w: 295, e: 307, n: 319 },
        Skins::Skeleton => Outfit {s: 286, w: 298, e: 310, n: 322 },
        Skins::Slime    => Outfit {s: 325, w: 337, e: 349, n: 361 },
        Skins::Bat      => Outfit {s: 328, w: 340, e: 352, n: 364 },
        Skins::Ghost    => Outfit {s: 331, w: 343, e: 355, n: 367 },
        Skins::Spider   => Outfit {s: 234, w: 246, e: 358, n: 370 },
    }
}
