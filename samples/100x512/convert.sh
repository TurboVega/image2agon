#!/bin/sh

# Should produce a 100x512 binary image (51200 bytes).
../image2agon -w 100 -h 512 >convert.log
cat convert.log

