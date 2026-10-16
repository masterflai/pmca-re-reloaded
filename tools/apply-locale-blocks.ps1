# Applies translated blocks to every catalogue in one pass.
#
# A block folder holds one `<lang>.ftl` per language, each with `key = text`
# lines. For every line: an existing key is replaced in place; a key listed in
# -Rename (new = old) replaces the old key's line; any other key is inserted
# after the -After key, or appended at the end when -After is not given.
#
# The script is idempotent, so a run that stopped half-way can simply be
# repeated. Do not pipe it into `Select-Object -First n`: that stops the script
# after n lines of output and leaves the remaining languages untouched.
#
# Afterwards run tools/check-locales.ps1: every language must end up with the
# same keys as en-US.
[CmdletBinding()]
param(
    [Parameter(Mandatory)] [string] $Blocks,
    [string] $After,
    [hashtable] $Rename = @{}
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$utf8 = New-Object System.Text.UTF8Encoding $false

foreach ($block in Get-ChildItem -LiteralPath $Blocks -Filter *.ftl) {
    $lang = $block.BaseName
    $target = Join-Path $root "locales\$lang\pmca.ftl"
    if (-not (Test-Path -LiteralPath $target)) { throw "no catalogue for $lang" }
    $text = [IO.File]::ReadAllText($target, $utf8)
    $insert = @()
    $lines = [IO.File]::ReadAllText($block.FullName, $utf8) -split "\r?\n" |
        Where-Object { $_ -match '^[a-z0-9-]+ = ' }
    foreach ($line in $lines) {
        $key = ($line -split ' = ')[0]
        $own = "(?m)^" + [regex]::Escape($key) + " = [^\r\n]*"
        $old = if ($Rename.ContainsKey($key)) { "(?m)^" + [regex]::Escape($Rename[$key]) + " = [^\r\n]*" } else { $null }
        $replace = [System.Text.RegularExpressions.MatchEvaluator] { param($m) $line }
        if ($old -and [regex]::IsMatch($text, $old)) {
            $text = [regex]::Replace($text, $old, $replace)
        } elseif ([regex]::IsMatch($text, $own)) {
            $text = [regex]::Replace($text, $own, $replace)
        } else {
            $insert += $line
        }
    }
    if ($insert.Count -gt 0) {
        $joined = $insert -join "`r`n"
        if ($After) {
            $anchor = "(?m)^" + [regex]::Escape($After) + " = [^\r\n]*"
            if (-not [regex]::IsMatch($text, $anchor)) { throw "$lang has no key $After" }
            $text = [regex]::Replace($text, $anchor, [System.Text.RegularExpressions.MatchEvaluator] { param($m) $m.Value + "`r`n" + $joined })
        } else {
            $text = $text.TrimEnd("`r", "`n") + "`r`n" + $joined + "`r`n"
        }
    }
    [IO.File]::WriteAllText($target, $text, $utf8)
    Write-Output ("{0}: {1} inserted" -f $lang, $insert.Count)
}
