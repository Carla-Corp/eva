@echo off
REM Usage: scripts\fix_symbols.cmd <library.lib>

set LIB_FILE=%1
set OLD_SYM=rust_eh_personality
set NEW_SYM=__custom_rust_eh

if "%LIB_FILE%"=="" (
    echo Usage: %0 ^<library.lib^>
    exit /b 1
)
if not exist "%LIB_FILE%" (
    echo Error: File not found: %LIB_FILE%
    exit /b 1
)

where llvm-ar >nul 2>nul || (
    echo Error: LLVM not found. Install from: https://releases.llvm.org/
    exit /b 1
)
where llvm-objcopy >nul 2>nul || (
    echo Error: llvm-objcopy not found
    exit /b 1
)

copy "%LIB_FILE%" "%LIB_FILE%.backup" >nul
set TEMP_DIR=%TEMP%\lib_fix_%RANDOM%
mkdir "%TEMP_DIR%" 2>nul
cd "%TEMP_DIR%"

llvm-ar x "%LIB_FILE%"
for %%f in (*.obj) do (
    llvm-objcopy --redefine-sym "%OLD_SYM%=%NEW_SYM%" "%%f"
)
llvm-ar rcs "%LIB_FILE%" *.obj

cd ..
rmdir /s /q "%TEMP_DIR%" 2>nul
echo Fixed: %LIB_FILE%
