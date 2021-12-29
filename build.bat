rustup run stable-i686-pc-windows-gnu cargo build --release

set EXT_PATH=C:\Users\daslocom\Documents\GameMaker\Projects\rope_demo.gmx\extensions
set DROP_PATH=C:\Users\daslocom\rope_lib\target\release
del "%EXT_PATH%\rope_lib.extension.gmx"
del "%EXT_PATH%\rope_lib\rope_lib.dll"
copy "%DROP_PATH%\rope_lib.dll" "%EXT_PATH%\rope_lib"
REM move "%EXT_PATH%\rope_lib\windows_lib.dll" "%EXT_PATH%\wrope_lib\orld_generators.dll"
copy "C:\users\daslocom\tmp\rope_lib.xml" "%EXT_PATH%"
move "%EXT_PATH%\rope_lib.xml" "%EXT_PATH%\rope_lib.extension.gmx"