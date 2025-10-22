@echo off
REM Build script for BlueDot GDExtension on Windows

echo Building BlueDot for Windows x86_64...

cargo build --release --target x86_64-pc-windows-msvc

echo Copying library to addons/bluedot/bin/windows-x86_64/...
if not exist "addons\bluedot\bin\windows-x86_64" mkdir "addons\bluedot\bin\windows-x86_64"
copy /Y "target\x86_64-pc-windows-msvc\release\bluedot.dll" "addons\bluedot\bin\windows-x86_64\bluedot.dll"

echo.
echo Build complete!
echo Library location: addons\bluedot\bin\windows-x86_64\bluedot.dll
echo.
echo Copy the entire addons\bluedot\ folder to your Godot project's addons\ directory
