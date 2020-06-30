![Rust](https://github.com/maikwoehl/realm.one/workflows/Rust/badge.svg)

# realm.one
![alt text](resources/img/screen3.png)

Realm one is an open source tile based game written in Rust using the Amethyst game engine. It is the first game that will be integrated into the distributed MMO platform [Worlds](https://github.com/Machine-Hum/Worlds). Following the implementation anyone will be able to fork this repository and add to the universe!

## Contribution
The project is under heavy development and we are always looking for people to help out! Please see the CONTRIBUTING document for development information. Join us on IRC! (#realm.one on Freenode), we also have a [Discord server]( https://discord.gg/ghJSrJk)!

### Tiles
Tiles are taken from [here](https://opengameart.org/content/tiny-16-basic?page=1) credit to Lanea Zimmerman! Tiles are arranged using the [Tiled Map Editor](https://www.mapeditor.org/).

## Running
You need to configure either metal or vulkan. This is shown in the cargo.Toml file.

### Client

```bash
git clone https://github.com/Machine-Hum/realm.one
cd realm.one
```

Setup the config file resources/config.ron, public server is **18.223.212.117:3457** right now 
```rust 
AppConfig(
    server_ip: "18.220.126.218:3457",
    client_ip: "Depricated, Can be blank",
    player_name: "YourName - Change this!",
)
```

```console
cargo run --release client 
```

### Server 

```bash
git clone https://github.com/Machine-Hum/realm.one
cd realm.one
```

Setup the config file resources/config.ron 
```rust
AppConfig(
    server_ip: "YourIP",
    client_ip: "",
    player_name: "",
)
```

```bash
cargo run --release server
```

## Gameplay
Once connected, you can move your character around with 'wasd' controls. There is also a command system that allows configuration of your character. To get into this command system press the '/' key, you can you freely type. Once you have finished your command press enter.

### Command listing
Commands are of the form: cmd arg subargs

```
cmd outfit male    // Change char to male
cmd outfit female  // Change char to female
cmd outfit nude    // Change char nude androgynous person 
```

Other outfits: bat, skeleton, slime, ghost, spider


## Hitlist (In order)
* [x] [F005 - Network Refactor](features/F005/)
* [x] [F006 - IO Refactor](features/F006/)
* [x] [F001 - Monster Instantiation](features/F001/)
* [x] [F015 - Monster AI](features/F015/)
* [ ] [F016 - Items](features/F016/)
* [ ] [F003 - Blood Splat during combat](features/F003/)
* [ ] [F009 - Other player actions should be visable](features/F009) 
* [ ] (machinehum) The Lifeformlist needs to remove ids from players and monsters when remove() is called.
* [ ] F014 - lil guy should have a health bar (Zelda Style hearts)
* [x] [F002 - Player UID](features/F002/)
* [x] F007 - Spritesheet should be modified to allow melee in all directions
* [ ] [F004 - Move Up/Down Stairs](features/F004/)
* [ ] F008 - Should be able to open and close doors
* [ ] F010 - Controls are clunky
* [ ] F011 - Serverside database
* [ ] F012 - Proper player authentication
* [ ] F013 - Build more maps
* [ ] Gameplay ideas
* [ ] A story lol

## Gotchas
- Make sure that all tiles on tiled have at least on property
