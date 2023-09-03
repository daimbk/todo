@echo off
mkdir "c:\aliases"
setx PATH "c:\aliases;%PATH%"

set CURRENT_DIR=%CD%
echo @echo off > "c:\aliases\todo.bat"
echo pushd "%CURRENT_DIR%" >> "c:\aliases\todo.bat"
echo "todo.exe" %%* >> "c:\aliases\todo.bat"
echo popd >> "c:\aliases\todo.bat"
