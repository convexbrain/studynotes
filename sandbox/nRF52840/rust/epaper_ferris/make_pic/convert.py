from PIL import Image
import numpy as np

img = np.array(Image.open('cuddlyferris.bmp'), np.float)

print(img.shape)

width = img.shape[1]
height = img.shape[0]

bpix = 0
bcnt = 0
black = np.zeros(0)

rpix = 0
rcnt = 0
red = np.zeros(0)

for y in range(height):
    for x in range(width):
        r = img[y, x, 0]
        g = img[y, x, 1]
        b = img[y, x, 2]
        img[y, x, :] = 255

        if r + g + b < 128:
            img[y, x, :] = 0
            p = 0
        else:
            p = 3
        bpix = (bpix << 2) + p
        bcnt = bcnt + 1
        if bcnt >= 4:
            black = np.append(black, bpix)
            bpix = 0
            bcnt = 0

        if r > g + b:
            img[y, x, 1] = 0
            img[y, x, 2] = 0
            p = 0
        else:
            p = 1
        rpix = (rpix << 1) + p
        rcnt = rcnt + 1
        if rcnt >= 8:
            red = np.append(red, rpix)
            rpix = 0
            rcnt = 0

black = np.reshape(black, (200, -1))
np.savetxt('black.txt', black, fmt='%3d,')
red = np.reshape(red, (200, -1))
np.savetxt('red.txt', red, fmt='%3d,')

Image.fromarray(img.astype(np.uint8)).save('a.png')
