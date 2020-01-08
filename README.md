# realm.one
![alt text](resources/img/screen3.png)

Realm one is an open source tile based game written in Rust using the Amethyst game engine. It is the first game that will be integrated into the distributed MMO platform [Worlds](https://github.com/Machine-Hum/Worlds). Following the implementation anyone will be able to fork this repository and add to the universe!

## Contribution
The project is under heavy development and we are always looking for people to help out! Please see the CONTRIBUTING document for development information and the [specs](https://github.com/Machine-Hum/realm.one/blob/master/spec/spec.pdf) document for an outline of what we're trying to build. Join us on IRC! (#realm.one on Freenode) [log here](https://freenode.logbot.info/realm.one). Feel free to reach out!

### Tiles
Tiles are taken from [here](https://opengameart.org/content/tiny-16-basic?page=1) credit to Lanea Zimmerman! Tiles are arranged using the [Tiled Map Editor](https://www.mapeditor.org/).

## Running
```bash
git clone https://github.com/Machine-Hum/realm.one
cd realm.one
cargo run --release server                # For the server
cargo run --release client 127.0.0.1:666  # For the client 

```

## Features
The game is under heavy development.

### Current Features
  - Map can be drawn with the Tiled map editor
  - The little guy can walk around, stay on the map and not walk through walls or off the map

### Future Features
  - little guy can go up and down stairs and to different sections of the map
  - MMO style, will be able to interact with other players
  - Will be able to fight monsters and gain exp
  - Will be able to equipt items
