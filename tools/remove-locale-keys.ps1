# Removes the given keys from every catalogue.
#
# Use tools/find-unused-keys.ps1 to find candidates; afterwards
# tools/check-locales.ps1 must report the same key count everywhere.
[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string[]] $Keys
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$utf8 = New-Object System.Text.UTF8Encoding $false
# `powershell -File` passes `-Keys a,b` as one string.
$Keys = @($Keys | ForEach-Object { $_ -split ',' } | ForEach-Object { $_.Trim() } | Where-Object { $_ })

foreach ($dir in Get-ChildItem (Join-Path $root "locales") -Directory) {
    $path = Join-Path $dir.FullName "pmca.ftl"
    $text = [IO.File]::ReadAllText($path, $utf8)
    $removed = 0
    foreach ($key in $Keys) {
        $pattern = "(?m)^" + [regex]::Escape($key) + " = [^\r\n]*\r?\n"
        $removed += [regex]::Matches($text, $pattern).Count
        $text = [regex]::Replace($text, $pattern, "")
    }
    [IO.File]::WriteAllText($path, $text, $utf8)
    Write-Output ("{0}: {1} removed" -f $dir.Name, $removed)
}
