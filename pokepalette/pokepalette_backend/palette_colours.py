import os
import sys
import numpy as np
from urllib.request import Request, urlopen
from skimage.io import imread, imsave
from sklearn.cluster import MiniBatchKMeans
from sklearn.cluster import KMeans


''' GET IMG LINK FROM SERVER (as sys.argv[1]) '''
POKE_IMG = sys.argv[1]
FILE_NAME = 'pokemon.png'
N_COLOURS = 8

# ******************************************************************************************
# ====================================== GET COLOURS =======================================
# ******************************************************************************************

''' REFERENCE: https://ggbaker.ca/data-science/code/cluster_palette.py '''

def getClusteredColours(POKE_IMG, FILE_NAME, N_COLOURS):
    req = Request(POKE_IMG, headers={'User-Agent': 'Mozilla/5.0'})

    try: 
        with urlopen(req) as imgdata, open(FILE_NAME, 'wb') as imgfile:
            imgfile.write(imgdata.read())
    except: 
        return 'ERROR'

    imgdata = imread(FILE_NAME)
    imgdata = imgdata[:, :, :3]
    imgdata = imgdata.reshape(-1, 3)

    model = MiniBatchKMeans(n_clusters=N_COLOURS, random_state=383, batch_size=1000)

    model.fit(imgdata)
    model.predict(imgdata)

    colours = model.cluster_centers_.astype(np.uint8)
    avg = np.average(colours, axis=1).reshape(-1, 1)

    colours = np.append(colours, avg, axis=1)

    ind = np.argsort(colours[:, -1])
    colours = colours[ind]

    ''' Keep the middle colours (drop the dark, outline colours, and whites from eye shading) '''

    colours = colours[2 : 7]
    colours = colours[:, :3].astype(int)

    return colours

# ******************************************************************************************
# ======================================== GET HEX =========================================
# ******************************************************************************************

''' REFERENCE: http://python.omics.wiki/plot/colors/rgb2hex '''

def RGBtoHex(rgb):
    red = rgb[0]
    green = rgb[1]
    blue = rgb[2]

    hx = '#{:02x}{:02x}{:02x}'.format(red, green, blue)
    hx = hx.upper()

    return hx


''' REFERENCE: https://www.niwa.nu/2013/05/math-behind-colorspace-conversions-rgb-hsl/ '''

def getHue(rgb):
    red = rgb[0] / 255.0
    green = rgb[1] / 255.0
    blue = rgb[2] / 255.0

    maxVal = max(red, green, blue)
    minVal = min(red, green, blue)

    chroma = maxVal - minVal
    hue = 0

    if chroma != 0:
        if maxVal == red:
            segment = (green - blue) / chroma

            if segment < 0:
                shift = 6
            else:
                shift = 0

            hue = segment + shift
        elif maxVal == green:
            segment = (blue - red) / chroma
            shift = 2

            hue = segment + shift
        else:
            segment = (red - green) / chroma
            shift = 4

            hue = segment + shift

    hue *= 60

    return hue


def getSaturation(rgb):
    red = rgb[0] / 255.0
    green = rgb[1] / 255.0
    blue = rgb[2] / 255.0

    maxVal = max(red, green, blue)
    minVal = min(red, green, blue)

    chroma = maxVal - minVal
    luminance = (maxVal + minVal) / 2.0
    saturation = 0

    if chroma == 0:
        return saturation

    if luminance <= 0.5:
        saturation = chroma / (maxVal + minVal)
    else: 
        saturation = chroma / (2.0 - maxVal - minVal)

    return saturation


def HSLtoRGB(hue, saturation, luminance):

    ''' REFERENCE: https://stackoverflow.com/questions/2353211/hsl-to-rgb-color-conversion/9493060#9493060 '''
    
    def hueToRGB(a, b, c):
        if c < 0:
            c += 1
        if c > 1:
            c -= 1
        
        if c < (1.0 / 6.0):
            return (a + (b - a) * 6 * c)
        elif c < (1.0 / 2.0):
            return b
        elif c < (2.0 / 3.0):
            return (a + (b - a) * (2.0 / 3.0 - c) * 6)

        return a

    red = green = blue = 0

    if saturation == 0:
        red = green = blue = luminance
    else: 
        if luminance < 0.5:
            b = luminance * (saturation + 1.0)
        else:
            b = luminance + saturation - luminance * saturation

        a = 2.0 * luminance - b

        red = min(round(hueToRGB(a, b, hue + (1.0 / 3.0)) * 255), 255)
        green = min(round(hueToRGB(a, b, hue) * 255), 255)
        blue = min(round(hueToRGB(a, b, hue - (1.0 / 3.0)) * 255), 255)

    return (red, green, blue)


def getComplementary(rgb):
    red = rgb[0] / 255.0
    green = rgb[1] / 255.0
    blue = rgb[2] / 255.0

    maxVal = max(red, green, blue)
    minVal = min(red, green, blue)

    hue = getHue(rgb)
    saturation = getSaturation(rgb)
    luminance = (maxVal + minVal) / 2

    ''' Shift hue 180 degrees to get complementary '''

    hue += 180

    if hue > 360:
        hue = hue % 360

    hue /= 360.0

    return HSLtoRGB(hue, saturation, luminance)


def getMonochromatic(rgb):
    red = rgb[0] / 255.0
    green = rgb[1] / 255.0
    blue = rgb[2] / 255.0

    maxVal = max(red, green, blue)
    minVal = min(red, green, blue)

    c = 0.10

    hue = getHue(rgb) / 360.0
    saturation = min(getSaturation(rgb) + c, 255)
    luminance = max((maxVal + minVal) / 2 - c, 0)

    return HSLtoRGB(hue, saturation, luminance)


def listToString(lst):
    sep = ', '
    hex_colours_str = sep.join(lst)

    return hex_colours_str

# ******************************************************************************************
# ===================================== PRINT OUTPUTS ======================================
# ******************************************************************************************

colours = getClusteredColours(POKE_IMG, FILE_NAME, N_COLOURS)

if colours == 'ERROR':
    print(colours)
else:
    hex_colours = []
    complementary_colours = []
    monochromatic_colours = []

    for c in colours:
        hex_colours.append(RGBtoHex(c))
        complementary_colours.append(getComplementary(c))
        monochromatic_colours.append(getMonochromatic(c))

    for c in complementary_colours:
        hex_colours.append(RGBtoHex(c))

    for c in monochromatic_colours:
        hex_colours.append(RGBtoHex(c))

    result = listToString(hex_colours)

    print(result)