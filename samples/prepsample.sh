# To be run from 'samples' directory.
clear
cd ..
cargo build --release
cd samples
rm PAINTING.BIN
rm PALETTE.BIN
rm painting.log
rm image2agon
ls -l
cp ../target/release/image2agon ./
./image2agon -w 320 -h 240 painting.png >painting.log
ls -l
hexdump -C PALETTE.BIN
gedit painting.log &

cd individual
../image2agon \
 -b 1 -p 15 monochrome.png \
 -a sp seq08.png \
 -a sp seq16.png \
 -a sp seq32.png \
 -a sp seq64.png  >individual.log
gedit individual.log &
cd ..

