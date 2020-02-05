# CONTRIBUTING
Quick notes on how this projects was made and how to contribute, to be improved in the future. This project uses the [Amethyst](https://amethyst.rs/) game engine.

Join #realm.one on Freenode if you want to chat!

## Map creation
Artwork comes from [OpenGameArt.org](https://opengameart.org/content/tiny-16-basic).

Maps are assembled from sprites using the ["Tiled" map editor](https://www.mapeditor.org/). Which yields `.tmx` files we can then parse in rust using the [rs-tiledcrate](https://github.com/mattyhall/rs-tiled). Tiles are combined into one massive tileset using TexturePacker, this tileset is called master16.png

## Keybindings
The keybindings are described in the `resources/bindings.ron` file. You also need to create the proper enum variants in `src/key_bindings.rs` for our game to be able to parse the file properly.

## Networking
The project uses Amethyst Network for client/server sync.

## TODO
There are lots of things that need to be worked on.

- Character Interaction
  - Chatting
  - Combat mechanics
  - Health Bar
 
- Player Movement
  - Up and Down stairs (changing map)
  - Player should animate during walking

- Monsters
  - Need to be implemented

- Items
  - Need to be implemented
 
- Creative
  - Need more tiles
  - Need more maps
  - Story/Gameplay
