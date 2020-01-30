use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub server_ip:   String,
    pub client_ip:   String,
    pub player_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self{ 
            server_ip:    "127.0.0.1:3456".to_string(),
            client_ip:    "127.0.0.1:3455".to_string(),
            player_name:  "Turnip".to_string(),
        } 
    }
}
