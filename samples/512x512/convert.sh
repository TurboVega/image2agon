#!/bin/sh

# Should produce a 512x512 binary image (262144 bytes).
../image2agon -w 512 -h 512 >convert.log
cat convert.log

