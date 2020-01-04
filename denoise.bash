#!/bin/bash

# Requires Downloads/oidn-1.1.0.x86_64.macos in downloads feild
# Requires imagemagick installed in system
# Modify for your use.

for f in ./*.png; do
    convert -endian LSB $f "${f%.png}.pfm" &&
    ~/Downloads/oidn-1.1.0.x86_64.macos/bin/denoise -ldr "${f%.png}.pfm" -o "${f%.png}-denoise.pfm" &&
    convert "${f%.png}-denoise.pfm" $f &&
    rm *.pfm
done
