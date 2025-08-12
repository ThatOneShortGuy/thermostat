@echo off
setlocal enableextensions enabledelayedexpansion

REM ========= Config (edit as needed) =========
set "TARGET=aarch64-unknown-linux-gnu"
set "BIN=client"
set "OUTDIR=thermostat-client"
set "REMOTE_HOST=braxt@thermostat.local"
set "REMOTE_DIR=~/"             REM where OUTDIR will be copied on the Pi
REM PROFILE: debug | release  (can override by passing "release" as the first arg)
set "PROFILE=debug"
REM ==========================================

if /I "%~1"=="release" set "PROFILE=release"

set "ARTIFACT=target\%TARGET%\%PROFILE%\%BIN%"
if /I "%PROFILE%"=="release" (set "RELEASE_FLAG=--release") else (set "RELEASE_FLAG=")

echo ==^> Checking for 'cross'...
where cross >nul 2>&1
if errorlevel 1 (
  echo [ERROR] 'cross' not found in PATH. Install with: cargo install cross
  exit /b 1
)

call :find_scp || exit /b 1
call :find_ssh || exit /b 1

echo ==^> Building (%PROFILE%) for %TARGET% ...
cross build %RELEASE_FLAG% --bin client --target %TARGET%
if errorlevel 1 (
  echo [ERROR] Build failed.
  exit /b 1
)

if not exist "%ARTIFACT%" (
  echo [ERROR] Build artifact not found: "%ARTIFACT%"
  echo         Ensure BIN="%BIN%" matches your binary name.
  exit /b 1
)

echo ==^> Preparing local output folder "%OUTDIR%" ...
if not exist "%OUTDIR%\" mkdir "%OUTDIR%"

echo ==^> Copying "%ARTIFACT%" to "%OUTDIR%\%BIN%" ...
copy /Y "%ARTIFACT%" "%OUTDIR%\%BIN%" >nul
if errorlevel 1 (
  echo [ERROR] Copy failed.
  exit /b 1
)

echo ==^> Uploading "%OUTDIR%" to %REMOTE_HOST%:%REMOTE_DIR% ...
"%SCP%" -r "%OUTDIR%" "%REMOTE_HOST%:%REMOTE_DIR%"
if errorlevel 1 (
  echo [ERROR] scp failed.
  exit /b 1
)

echo ==^> Running "%BIN%" on %REMOTE_HOST% ...
REM We ensure executable bit, cd, then run in foreground.
REM SSH session will end when the program exits.
"%SSH%" "%REMOTE_HOST%" "mkdir -p ~/thermostat-client && cd ~/thermostat-client && chmod +x %BIN% && sudo ./\%BIN%"
set "rc=%ERRORLEVEL%"
echo ==^> Remote program exited with code %rc%.
exit /b %rc%


:find_scp
where scp >nul 2>&1
if %errorlevel% EQU 0 (
  for /f "usebackq delims=" %%i in (`where scp`) do set "SCP=%%i"
  goto :eof
)
if exist "%SystemRoot%\System32\OpenSSH\scp.exe" (
  set "SCP=%SystemRoot%\System32\OpenSSH\scp.exe"
  goto :eof
)
if exist "C:\Program Files\Git\usr\bin\scp.exe" (
  set "SCP=C:\Program Files\Git\usr\bin\scp.exe"
  goto :eof
)
echo [ERROR] 'scp' not found. Install "OpenSSH Client" (Windows Optional Features)
echo         or ensure Git for Windows provides scp and is on PATH.
exit /b 1

:find_ssh
where ssh >nul 2>&1
if %errorlevel% EQU 0 (
  for /f "usebackq delims=" %%i in (`where ssh`) do set "SSH=%%i"
  goto :eof
)
if exist "%SystemRoot%\System32\OpenSSH\ssh.exe" (
  set "SSH=%SystemRoot%\System32\OpenSSH\ssh.exe"
  goto :eof
)
if exist "C:\Program Files\Git\usr\bin\ssh.exe" (
  set "SSH=C:\Program Files\Git\usr\bin\ssh.exe"
  goto :eof
)
echo [ERROR] 'ssh' not found. Install "OpenSSH Client" or add it to PATH.
exit /b 1
