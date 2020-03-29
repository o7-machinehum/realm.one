use serde::{Serialize, Deserialize};

pub fn outfit_from_str(skin: String) -> Outfit {
    match skin.as_str() {
     "Nude"     => get_outfit(&Skins::Nude),    
     "Male"     => get_outfit(&Skins::Male),  
     "Female"   => get_outfit(&Skins::Female),  
     "Skeleton" => get_outfit(&Skins::Skeleton),
     "Slime"    => get_outfit(&Skins::Slime),
     "Bat"      => get_outfit(&Skins::Bat), 
     "Ghost"    => get_outfit(&Skins::Ghost),   
     "Spider"   => get_outfit(&Skins::Spider),
     _          => get_outfit(&Skins::Nude)
    }
}

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
    pub at: AttackStance,
}

pub fn get_outfit(skin: &Skins) -> Outfit {
    match skin {
        Skins::Nude     => Outfit {s: 277, w: 289, e: 301, n: 313, at: AttackStance::new(skin) },
        Skins::Male     => Outfit {s: 280, w: 292, e: 304, n: 316, at: AttackStance::new(skin) },
        Skins::Female   => Outfit {s: 283, w: 295, e: 307, n: 319, at: AttackStance::new(skin) },
        Skins::Skeleton => Outfit {s: 286, w: 298, e: 310, n: 322, at: AttackStance::new(skin) },
        Skins::Slime    => Outfit {s: 325, w: 337, e: 349, n: 361, at: AttackStance::new(skin) },
        Skins::Bat      => Outfit {s: 328, w: 340, e: 352, n: 364, at: AttackStance::new(skin) },
        Skins::Ghost    => Outfit {s: 331, w: 343, e: 355, n: 367, at: AttackStance::new(skin) },
        Skins::Spider   => Outfit {s: 334, w: 346, e: 358, n: 370, at: AttackStance::new(skin) },
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AttackStance {
    pub n: usize,      
    pub e: usize, 
    pub s: usize,
    pub w: usize, 
    pub s_n: usize,  //Sword North      
    pub s_e: usize, 
    pub s_s: usize,
    pub s_w: usize, 
}

impl AttackStance {
    pub fn new(skin: &Skins) -> Self {
        match skin {
            _ => AttackStance {n: 580, e: 614, s: 640, w: 614, s_n: 587, s_e: 615, s_s: 652, s_w: 615}
            // Skins::Nude     => Outfit {s: 277, w: 289, e: 301, n: 313 },
            // Skins::Male     => Outfit {s: 280, w: 292, e: 304, n: 316 },
            // Skins::Female   => Outfit {s: 283, w: 295, e: 307, n: 319 },
            // Skins::Skeleton => Outfit {s: 286, w: 298, e: 310, n: 322 },
            // Skins::Slime    => Outfit {s: 325, w: 337, e: 349, n: 361 },
            // Skins::Bat      => Outfit {s: 328, w: 340, e: 352, n: 364 },
            // Skins::Ghost    => Outfit {s: 331, w: 343, e: 355, n: 367 },
            // Skins::Spider   => Outfit {s: 234, w: 246, e: 358, n: 370 },
        }
    }
}

//pub fn get_directed(stance: AttackStance, or: Orientation) -> (usize, usize){
//    match or {
//        Orientation::North => return (stance.n, stance.s_n),
//        Orientation::East  => return (stance.e, stance.s_e),
//        Orientation::South => return (stance.s, stance.s_s),
//        Orientation::West  => return (stance.w, stance.s_w),
//    }
//}
