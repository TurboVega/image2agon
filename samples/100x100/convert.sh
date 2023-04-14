#!/bin/sh

# Should produce a 100x100 binary image (10000 bytes).
../image2agon -w 100 -h 100 >convert.log
cat convert.log


