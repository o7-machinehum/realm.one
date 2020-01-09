use serde::{Serialize, Deserialize};
use serde_json;

// eg:
// let mut k = network::pack::new(10);
// let p = k.to_string();
// info!("{}", p);
// let t = network::pack::from_string(p);
// info!("{:?}", t);

#[derive(Serialize, Deserialize, Debug)]
pub struct pack{
    id: u32,
}

impl pack{
    pub fn new(id: u32) -> Self {
        Self {
            id,
        }
    }

    pub fn from_string(str: String) -> Self {
       serde_json::from_str(&str).unwrap()
    }
     
    pub fn to_string(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
