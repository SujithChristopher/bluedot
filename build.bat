@echo off
REM Build script for GdBLE GDExtension on Windows

echo Building GdBLE for Windows x86_64...

cargo build --release --target x86_64-pc-windows-msvc

echo Copying library to addons/gdble/bin/windows-x86_64/...
if not exist "addons\gdble\bin\windows-x86_64" mkdir "addons\gdble\bin\windows-x86_64"
copy /Y "target\x86_64-pc-windows-msvc\release\gdble.dll" "addons\gdble\bin\windows-x86_64\gdble.dll"

echo.
echo Build complete!
echo Library location: addons\gdble\bin\windows-x86_64\gdble.dll
echo.
echo Copy the entire addons\gdble\ folder to your Godot project's addons\ directory
