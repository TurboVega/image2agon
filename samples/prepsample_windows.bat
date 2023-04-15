REM To be run from 'samples' directory.
cls
cd ..
cargo build --release
cd samples
del *.BIN
del *.RGB
del *.log
del image2agon.exe
dir
copy ..\target\release\image2agon.exe .\*
.\image2agon.exe -w 320 -h 240 painting.png >painting.log
dir
start notepad painting.log &

cd individual
del *.BIN
del *.RGB
del *.log
..\image2agon.exe ^
 -b 1 monochrome.png ^
 seq08.png ^
 seq16.png ^
 seq32.png ^
 seq64.png  >individual.log
start notepad individual.log
cd ..
