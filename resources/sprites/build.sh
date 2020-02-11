#!/bin/bash

convert textpack/master.png textpack/10Tile-B.png -append temp.png 
convert temp.png textpack/04Tiny16-ExpandedMaleSprites.png -append temp.png 
convert temp.png textpack/05Tiny16-ExpandedBaseSprites_Nude.png -append temp.png 
convert temp.png textpack/07Tiny16-ExpandedFemaleSprites.png -append temp.png 
convert temp.png textpack/08Tiny16-ExpandedSkeletonSprites.png -append temp.png 
# Add new converts here

mv temp.png master16.png
