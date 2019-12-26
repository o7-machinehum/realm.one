# CONTRIBUTING

Quick notes on how this projects was made and how to contribute, to be
improved in the future.

This project uses the [Amethyst](https://amethyst.rs/) game engine.


## Map creation

Artwork comes from [OpenGameArt.org](https://opengameart.org/content/tiny-16-basic).

Maps are assembled from sprites using the ["Tiled" map editor](https://www.mapeditor.org/). Which
yields `.tmx` files we can then parse in rust using the [rs-tiled
crate](https://github.com/mattyhall/rs-tiled).

## Keybindings

The keybindings are described in the `resources/bindings.ron` file.
You also need to create the propper enum variants in `src/key_bindings.rs` for our game to be able
to parse the file properly.
