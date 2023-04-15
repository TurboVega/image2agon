# To be run from 'samples' directory.
clear
cd ..
cargo build --release
cd samples
rm *.BIN
rm *.RGB
rm *.log
rm image2agon
ls -l
cp ../target/release/image2agon ./
./image2agon -w 320 -h 240 painting.png >painting.log
ls -l
hexdump -C PALETTE.BIN
gedit painting.log &

cd individual
rm *.BIN
rm *.RGB
rm *.log
../image2agon \
 -b 1 monochrome.png \
 seq08.png \
 seq16.png \
 seq32.png \
 seq64.png  >individual.log
gedit individual.log &
cd ..
