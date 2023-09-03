@echo off
setlocal enabledelayedexpansion

:: Check if %USERPROFILE%\bin directory exists
if exist "%USERPROFILE%\bin\" (
    :: Check if %USERPROFILE%\bin is already in PATH
    set "pathExists=false"
    for %%I in ("%PATH:;=" "%") do (
        if "%%I"=="%USERPROFILE%\bin" (
            set "pathExists=true"
            exit /b
        )
    )
    
    :: If %USERPROFILE%\bin is not in PATH, add it
    if !pathExists!==false (
        setx PATH "%USERPROFILE%\bin;%PATH%" /m
        echo Added %USERPROFILE%\bin to PATH.
    ) else (
        echo %USERPROFILE%\bin is already in PATH. No changes were made.
    )
) else (
    echo The %USERPROFILE%\bin directory does not exist. Please create it and place your scripts there.
)

endlocal
