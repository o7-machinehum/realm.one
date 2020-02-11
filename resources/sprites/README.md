# Cheat Sheet
If you want to change the master16.png there are three things you must must do!
  - Where indicated in the build.sh, add another line merging in your new spritesheet
  - Ensure the all new sprites have been placed BELOW the old master16.png, this is to ensure the sprite number do _not_ change
  - Change the number of sprites constant inside spritescontainer.rs! 
  - Change master16.ron to reflect the new resolution!

## Padding pixels on an image

This will pad 1 pixel to the east side of textpack/10Tile-B.png

```
convert textpack/10Tile-B.png -background none -gravity east -splice 1x0 result.png
```

## Combining spritesheets

This will add textpack/10Tile-B.png to the bottom of master16.png
```
convert master16.png textpack/10Tile-B.png -append out.png
```


## Removing Deadspace
This will remove any dead pixels

```
convert -trim Tile-B.png 10Tile-B.png
```

## Resizeing
This will resize

```
convert 10Tile-B.png -resize 128x224 10Tile-B.png
```


## Removing Background
This will make the background transparent, note pixel at position 0,0 _must_ be the colour that you want to make transparent

```
convert 07Tiny16-ExpandedFemaleSprites.png -fill none -draw "matte 0,0 replace" result.png
```
