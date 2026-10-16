# Reports catalogue entries that are still byte-identical to the English text.
#
# A long string that matches en-US exactly is almost always an untranslated
# leftover rather than a coincidence; short ones (`OK`, `GPS`, proper nouns)
# legitimately agree across languages, so a length floor keeps the noise down.
[CmdletBinding()]
param(
    [int] $MinLength = 25
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$localeRoot = Join-Path $repoRoot 'locales'

function Read-Catalog([string] $path) {
    $map = [ordered]@{}
    foreach ($line in [System.IO.File]::ReadAllLines($path)) {
        if ($line -match '^([a-z0-9-]+)\s*=\s*(.+)$') {
            $map[$Matches[1]] = $Matches[2].Trim()
        }
    }
    return $map
}

$reference = Read-Catalog (Join-Path $localeRoot 'en-US\pmca.ftl')
Write-Host "en-US: $($reference.Count) keys"

foreach ($dir in Get-ChildItem $localeRoot -Directory | Where-Object Name -ne 'en-US') {
    $catalog = Read-Catalog (Join-Path $dir.FullName 'pmca.ftl')
    $hits = @()
    foreach ($key in $catalog.Keys) {
        $value = $catalog[$key]
        if ($value.Length -lt $MinLength) { continue }
        # -ceq: a case difference is still a translation decision.
        if ($reference.Contains($key) -and $reference[$key] -ceq $value) {
            $hits += $key
        }
    }
    if ($hits.Count -gt 0) {
        Write-Host ("{0,-8} {1,3} untranslated" -f $dir.Name, $hits.Count)
        foreach ($k in $hits) { Write-Host "           $k" }
    }
}
