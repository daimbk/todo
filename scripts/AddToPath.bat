@echo off
setlocal

:: Get the directory of the batch script
set "script_dir=%~dp0"

:: Specify the name of your Rust executable (without extension)
set "rust_exe=todo"

:: Construct the full path to the Rust executable
set "rust_path=%script_dir%%rust_exe%.exe"

:: Check if the Rust executable exists
if exist "%rust_path%" (
    echo Found Rust executable: %rust_path%
    set RUST_PATH=%rust_path%
) else (
    echo Rust executable not found in the same folder as the batch script.
    echo Please make sure the executable is in the same directory.
    exit /b 1
)

:: Your other batch script commands go here

endlocal
