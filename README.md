# image2agon
Converts PNG files to binary data for AgonLight (TM) usage.

This document is for version V1.5 of the program.

V1.0 - initial upload<br>

This program converts PNG file data into binary data for use on the
AgonLight (TM) retro computer. It reads multiple PNG files, combines their needed
color palettes, and outputs palette entries in both text and binary, plus it outputs binary pixel data (palette indexes and also color values). Additionally,
the program arranges a simple memory map, and outputs that information,
which may be helpful in loading the binary data into RAM.

NOTE: Even though the program can emit 1, 2, 3, 4, or 6 bits-per-pixel in the <i>output</i> for Agon, the <i>input</i> PNG files may contain 24-bit RGB or 32-bit RGBA data. It can also do
24-bit and 32-bit output.

The output palette will always be a set of 64 (or less) 6-bit colors,
meaning that it represents (up to) 63 colors out of a set of 64 possible
colors. Color <i>index</i> 0 always represents transparency.
After the program consolidates colors into palette indexes, trailing unused palette indexes may be used at your discretion.

The palette contents are printed to the console (and can be piped to a file), as assembler source text, and as BASIC source text.
The palette contents are written to the "PALETTE.BIN" file, and can be loaded into RAM using the ??? command in BASIC.

The output bitmap (set of palette indexes representing colors) can also be loaded
into RAM using the ??? command.

The program can be run in one of 3 ways. If no directory is specified, or if
the current directory is specified, the app searches for PNG files in the
current directory.

If a list of one or more directories is specified, the app
searches for files in those directories. In either case, all files are
processed together, where the resulting palette is concerned, so that any of
the images can be displayed on the Agon, using the resulting palette,
assuming that a proper video mode is used.

The third way is that you can specify individual PNG files, instead of or along
with other directories. This can be quite useful when trying to arrange files
with different purposes (such as tiles versus sprites) into a memory map
in a custom order.

Note: This program does not recursively traverse directories. To process subdirectories,
run the program multiple times, with different command line arguments.

The command-line format for this program is as follows:

```
image2agon { [-w width] [-h height] [-b <1|2|4|8>] [-n] [ <dir2|png2> | ./] } ...
```

??? multi-frame bitmaps ???

'-w' and '-width' are synonyms (either one is allowed)<br>
For a PNG file (in a directory or specific), 'width' is given in pixels.
<br>
'-h' and '-height' are synonyms<br>
For a PNG file (in a directory or specific), 'height' is given in pixels.
<br>
'b' and '-bpp' are synonyms<br>
This may be used to specify the number of bits per pixel in the output binary
file, which provides the intended range of color indexes (1: 2 colors, 2: 4
colors, 3: 8 colors, 4: 16 colors, 6: 64 colors). The default value is 6, for 64 colors.
As always, color index #0 means transparent, so the actual number of colors is
one less than the range might imply.<br>
<br>
If you use the same palette offset index (i.e., share it)
for multiple input files, be sure to list
the files in order of their bits-per-pixel numbers, from lowest to highest. For example,
if a 2-bpp file and a 4-bpp file share the same palette offset, list the 2-bpp file
before the 4-bpp file, in the command line.<br>
<br>
'-n' and '-nooutput' are synonymns<br>
When this option is specified, the output file will not exist, meaning that there will
be no output file for the given input image. This option may be used simply to modify
the color palette.<br>
<br>
<br>
'dir1' and 'dir2' are names or paths of directories<br>
<br>
'png1' and 'png2' are names or paths of individual PNG files<br>
<br>
As an example of changing image size, the "painting.png" file in the "samples"" directory of this project was
processed using "-w 320 -h 240" as the command parameters (note the spaces), to yield the BIN file in that same directory. Here is the entire command line:

```
./image2agon -w 320 -h 240 painting.png >painting.log
```

The image can be displayed using the following steps:

* step1
* step2
* After BASIC loads to its initial screen, load and run "PAINTING.BAS".

Another example illustrates specifying individual files, rather than directories.

The "individual" sample directory may be processed like this:

```
../image2agon \
 -b 1 monochrome.png \
 seq08.png \
 seq16.png \
 seq32.png \
 seq64.png  >individual.log
```

The resulting log file will contain a RAM memory map, such as the following.
The program attempts to arrange memory with the least possible amount of waste.

```
RAM Address Arrangement

Waste Start  End    Size  Width Height Path/Name
----- ------ ------ ----- ----- ------ ----------------------------------

```

In the above example, there is one file that is processed with 1 bit-per-pixel color.
The PNG file contains pixels that are either white or transparent (there is no black).
The resulting palette contains the following lines in it. This implies
that any '0' bits in the MONOCHROME.PNG file will show as transparent, and any '1'
bits in that file will show as white (color index #1 is RGB(3,3,3)), in the palette.

```
    ...
    ...
```

There are also other example conversions of the 'painting' file, as shell scripts, in sub-directories off of the "samples" directory. If you run a script,
it should convert the image file to binary, based on the command line in
the script. Some of the output sizes generated by these sample scripts are
physically too large to fit inside the RAM, but using the output is not
the point; these are just examples to show how the command line affects
the output data.

On a <i>per-input</i> (file or directory) basis, you may choose to specify the output width and/or height, in pixels.
If neither width nor height is specified, then the width and height are taken from
the input files. If one or both dimensions are specified, then the output pixel data
(palette map indexes) is sized accordingly, either by padding with transparent pixels,
or by cropping (discarding) extra pixels. The input is always centered over the output.

For example, using an input image of 57x64 pixels (width x height), and a command
line option "-w 64", the output image will be 64x64 pixels, because the height is
taken from the input image file. Specifying "-w 640 -h 480" for the same input image
will result in the original, small image being centered in a 640x480 space.

NOTE: This program does <b>not</b> resize an image by stretching or shrinking it, and it does <b>not</b> attempt to optimize the palette, such as converting an image with 152 colors
into an image with just 63 colors. The only color
conversion that is does is to take 24-bit RGB data, and right-shift each of
the color components by 6 (i.e., divide by 64), to yield a 6-bit color from the input 24-bit color.
This implies that detail may be lost, if the original image had non-zero
values in the least significant 6 bits of any color component of any pixel.

The overall processing is as follows:
* Obtain a list of all files to process.
* Read all files.
* Determine how many unique 6-bit colors are used in ALL files together.
* Organize a new color palette (index 0 means transparent; indexes 1..63 mean color).
* Output palette information as binary data.
* Output palette information as source text.
* Output image data as binary palette indexes, one index per pixel.
* Compute and output memory map as text.

NOTE: Regardless of which portion (some or all) of each input file is copied
(either in whole or in part) to the output, the <b>entire</b> input image is used to determine the combined palette. The main intent of this program is
to create a single palette that can be used for multiple images, tiles,
and/or sprites, so that they can all be shown on a single screen.
