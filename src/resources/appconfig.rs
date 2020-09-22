use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub server_ip:   String,
    pub client_ip:   String,
    pub player_name: String,

    pub tile_size: f32,
    pub player_move: f32,
    pub tile_per_player: f32,
    pub action_delay_ms: u128,
    pub typing_delay_ms: u128,
    pub player_speed_ms: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self{ 
            server_ip:        "127.0.0.1:3456".to_string(),
            client_ip:        "127.0.0.1:3455".to_string(),
            player_name:      "Turnip".to_string(),
            tile_size:        8.0,
            player_move:      16.0,
            tile_per_player:  16.0 / 8.0,
            action_delay_ms:  500,
            typing_delay_ms:  150,
            player_speed_ms:  500.0,
        } 
    }
}
