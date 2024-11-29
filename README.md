# RVST

This is a malware created for a lab in COMP 3 - Advanced Investigations.

The purpose is to try and get around windows defender and hopefully inject some malicious code into a service and run it.

This malware creates a simple dialog message if successful.





```


```powershell
$service = Get-Service -Name "AnotherService"
$pid = $service.Id
New-Service -Name "Rust Service" -BinaryPathName "C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe  "C:\Temp\oxidation.exe  $(Get-Process -Name 'notepad' ).Id'"


```


```