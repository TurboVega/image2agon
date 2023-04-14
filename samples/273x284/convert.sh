#!/bin/sh

# Should produce a 273x284 binary image (77532 bytes).
../image2agon >convert.log
cat convert.log

