@echo off

echo Building...
rustup run stable-i686-pc-windows-gnu cargo build --release
echo Building done

set EXT_PATH=C:\Users\Dan\Documents\GameMaker\Projects\rope_demo.gmx\extensions
set EXT_PATH_OTHER=C:\Users\Dan\Documents\GameMaker\Projects\LD49_cornwall_3.gmx\extensions
set DROP_PATH=C:\Users\Dan\rope_lib\target\release

echo Copying to %EXT_PATH%
del "%EXT_PATH%\rope_lib.extension.gmx"
del "%EXT_PATH%\rope_lib\rope_lib.dll"
copy "%DROP_PATH%\rope_lib.dll" "%EXT_PATH%\rope_lib"
REM move "%EXT_PATH%\rope_lib\windows_lib.dll" "%EXT_PATH%\wrope_lib\orld_generators.dll"
copy "C:\users\Dan\tmp\rope_lib.xml" "%EXT_PATH%"
move "%EXT_PATH%\rope_lib.xml" "%EXT_PATH%\rope_lib.extension.gmx"
echo Done

echo Copying to %EXT_PATH_OTHER%
del "%EXT_PATH_OTHER%\rope_lib.extension.gmx"
del "%EXT_PATH_OTHER%\rope_lib\rope_lib.dll"
copy "%DROP_PATH%\rope_lib.dll" "%EXT_PATH_OTHER%\rope_lib"
REM move "%EXT_PATH%\rope_lib\windows_lib.dll" "%EXT_PATH%\wrope_lib\orld_generators.dll"
copy "C:\users\Dan\tmp\rope_lib.xml" "%EXT_PATH_OTHER%"
move "%EXT_PATH_OTHER%\rope_lib.xml" "%EXT_PATH_OTHER%\rope_lib.extension.gmx"
echo Done