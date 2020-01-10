use crate::network;
use crate::network::Pack;
use log::info;

fn welcome() -> Pack {
    info!("Player Connected, sending map!");
    Pack::send_tmx("map1".to_string(), "Map contents".to_string())
}

pub fn handle(str: String) -> Pack {
    let pk = network::Pack::from_string(str);
    match pk.cmd {
        network::Cmd::Nothing       => {},
        network::Cmd::TransferMap   => {}, 
        network::Cmd::Connect       => return welcome(),
        network::Cmd::CreateMonster => {},
    }
    network::Pack::nothing() 
}
