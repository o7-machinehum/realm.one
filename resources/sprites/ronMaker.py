#eg: python3 ronMaker.py <out_fname.ron> <texture_width> <texture_height> <width> <height>

import sys

fname              = sys.argv[1]  
texture_width      = int(sys.argv[2]) 
texture_height     = int(sys.argv[3])
width              = int(sys.argv[4])
height             = int(sys.argv[5])
cols               = int(width / texture_width)
rows               = int(height / texture_height)

print("Output File Name: %s" %(fname));
print("Texture Width/Height (px): %d/%d" %(texture_width, texture_height));
print("Image Width/Height (px): %d/%d" %(width, height));
print("Sprites cols/rows: %d/%d" %(cols, rows));

f = open(fname, "w+")

f.write('(\n')
f.write('texture_width: ' + str(texture_width) +  ',\ntexture_height: ' + str(texture_height) + ',\nsprites: [', )

x = 0
y = 0

for ii in range(0, rows):
    for i in range(0, cols):
        f.write('\n(');
        f.write('\nx: ' + str(x))
        f.write(',\ny: ' + str(y))
        f.write(',\nwidth: ' + str(width))
        f.write(',\nheight: ' + str(height))
        if x == (texture_width - width)  and y == (texture_height - height): 
            f.write(',\n)');
        else: 
            f.write(',\n),');
       
        x += width
    y += height
    x = 0

f.write('\n]')
f.write('\n)')
