#eg: python3 ronMaker.py <fname.ron> <img_width> <img_height> <texture_width> <texture_height>
#eg: python3 ronMaker.py master16.ron 192 496 16 16

import sys

fname              = sys.argv[1]  
texture_width      = int(sys.argv[2]) 
texture_height     = int(sys.argv[3])
width              = int(sys.argv[4])
height             = int(sys.argv[5])
cols               = int(texture_width   / width)
rows               = int(texture_height  / height)

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
