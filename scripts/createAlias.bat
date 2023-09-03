@echo off

:: check if c:\aliases is already in the PATH
echo %PATH% | findstr /i "\<c:\\aliases\>" > nul
if errorlevel 1 (
    setx PATH "c:\aliases;%PATH%"
)

mkdir "c:\aliases"
set CURRENT_DIR=%CD%
echo @echo off > "c:\aliases\todo.bat"
echo pushd "%CURRENT_DIR%" >> "c:\aliases\todo.bat"
echo "todo.exe" %%* >> "c:\aliases\todo.bat"
echo popd >> "c:\aliases\todo.bat"
