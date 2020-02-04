# realm.one
![alt text](resources/img/screen3.png)

Realm one is an open source tile based game written in Rust using the Amethyst game engine. It is the first game that will be integrated into the distributed MMO platform [Worlds](https://github.com/Machine-Hum/Worlds). Following the implementation anyone will be able to fork this repository and add to the universe!

## Pre-Alpha Release
Join on #realm.one on Freenode Monday February 3rd, 6PM PST for more info! Lets all run around togeather in game!

## Contribution
The project is under heavy development and we are always looking for people to help out! Please see the CONTRIBUTING document for development information. Join us on IRC! (#realm.one on Freenode) [log here](https://freenode.logbot.info/realm.one). 

### Tiles
Tiles are taken from [here](https://opengameart.org/content/tiny-16-basic?page=1) credit to Lanea Zimmerman! Tiles are arranged using the [Tiled Map Editor](https://www.mapeditor.org/).

## Running

### Client

```bash
git clone https://github.com/Machine-Hum/realm.one
cd realm.one
```

Setup the config file resources/config.ron, public server is **18.220.126.218:3457** right now 
```
AppConfig(
    server_ip: "18.220.126.218:3457",
    client_ip: "Depricated, Can be blank",
    player_name: "YourName - Change this!",
)
```

```bash
cargo run --release client 
```

### Server 

```bash
git clone https://github.com/Machine-Hum/realm.one
cd realm.one
```

Setup the config file resources/config.ron 
```
AppConfig(
    server_ip: "YourIP",
    client_ip: "",
    player_name: "",
)
```

```bash
cargo run --release server
```
