# Checks every catalogue against en-US: key count, key parity, BOM and line endings.
#
# The parity test in pmca-i18n already fails the build on a missing key, but it
# reports one locale at a time. This gives the whole picture in one pass, and it
# also covers the two things the Rust test cannot see: a byte-order mark, which
# breaks gitleaks, and a bare LF, which the catalogues do not use.

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot

function Get-Keys([string]$Path) {
    Get-Content -LiteralPath $Path -Encoding UTF8 |
        Where-Object { $_ -match '^[a-z0-9-]+ =' } |
        ForEach-Object { ($_ -split ' =')[0] }
}

$reference = Get-Keys (Join-Path $root "locales\en-US\pmca.ftl")
Write-Output ("en-US: {0} keys" -f $reference.Count)

foreach ($dir in Get-ChildItem (Join-Path $root "locales") -Directory) {
    $path = Join-Path $dir.FullName "pmca.ftl"
    $keys = Get-Keys $path
    $diff = @(Compare-Object $reference $keys)

    $bytes = [IO.File]::ReadAllBytes($path)
    $bom = ($bytes.Length -ge 3 -and $bytes[0] -eq 0xEF -and $bytes[1] -eq 0xBB -and $bytes[2] -eq 0xBF)

    $text = [IO.File]::ReadAllText($path)
    $crlf = ([regex]::Matches($text, "`r`n")).Count
    $bareLf = ([regex]::Matches($text, "(?<!`r)`n")).Count

    Write-Output ("{0,-6} keys={1,-5} diff={2,-4} bom={3,-6} crlf={4,-5} bareLF={5}" -f `
            $dir.Name, $keys.Count, $diff.Count, $bom, $crlf, $bareLf)

    foreach ($entry in $diff) {
        $side = if ($entry.SideIndicator -eq "<=") { "fehlt" } else { "zusaetzlich" }
        Write-Output ("       {0}: {1}" -f $side, $entry.InputObject)
    }
}
