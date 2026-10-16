# Lists catalogue keys that no source file mentions.
#
# Report only, not a gate: some keys are built at runtime (`err-<code>`,
# `tweak-<id>`, `<key>-hint`) and show up here although they are used. Check
# each candidate by hand before removing it from all 21 catalogues.
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot

$keys = Get-Content -LiteralPath (Join-Path $root "locales\en-US\pmca.ftl") -Encoding UTF8 |
    Where-Object { $_ -match '^[a-z0-9-]+ =' } |
    ForEach-Object { ($_ -split ' =')[0] }

$sources = @(
    Get-ChildItem -Path (Join-Path $root "crates") -Recurse -Include *.rs, *.ts, *.html |
        Where-Object { $_.FullName -notmatch '\\(target|node_modules|dist|bindings)\\' }
)
$corpus = ($sources | ForEach-Object { [IO.File]::ReadAllText($_.FullName) }) -join "`n"

# Families whose keys are assembled at runtime from a prefix and a code.
$dynamic = '^(err-|mode-|check-(apps|gps|updater)-|shutter-|catalog-problem-|settings-theme-|elevation-[a-z]+-hint$)'

$unused = @($keys | Where-Object { -not $corpus.Contains($_) -and $_ -notmatch $dynamic })
foreach ($key in $unused) { Write-Output $key }
Write-Output ("{0} of {1} keys not found literally in crates/" -f $unused.Count, $keys.Count)
