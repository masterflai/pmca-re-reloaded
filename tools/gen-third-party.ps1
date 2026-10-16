#Requires -Version 5.1
<#
.SYNOPSIS
    Regenerates THIRD-PARTY.md from the Cargo dependency graphs.

.DESCRIPTION
    MIT and Apache-2.0 both require that the copyright notice and the licence
    text travel with a *binary* distribution, not just with the source. This
    project ships installers and a portable archive, so the notices have to be
    in the package. Collecting them by hand would mean a file that is correct
    on the day it is written and wrong at the next dependency bump, which is
    why this exists as a script instead.

    Two graphs are read, because the repository has two independent
    workspaces: the root one, and `crates/pmca-gui/src-tauri`, which Cargo does
    not reach from the root. A package present in both is listed once.

    Copyright lines are taken from each crate's own licence file in the local
    registry cache, so what appears here is what the author wrote rather than
    something inferred from the SPDX identifier. A crate whose sources are not
    unpacked is still listed, with its licence, but without a notice line — and
    is counted in the summary, so the omission is visible rather than silent.

.EXAMPLE
    pwsh tools/gen-third-party.ps1

    Writes THIRD-PARTY.md in the repository root.
#>
[CmdletBinding()]
param(
    # Where to write the result. Defaults to THIRD-PARTY.md beside this repo.
    [string] $OutFile
)

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
if (-not $OutFile) { $OutFile = Join-Path $repoRoot 'THIRD-PARTY.md' }

$workspaces = @(
    @{ Name = 'pmca (CLI, core, USB, market)'; Path = $repoRoot }
    @{ Name = 'pmca-gui (Tauri desktop shell)'; Path = (Join-Path $repoRoot 'crates\pmca-gui\src-tauri') }
)

# name+version -> record. Keyed that way so two workspaces pulling the same
# crate at different versions are both reported; the same version twice is not.
$packages = [ordered]@{}

foreach ($ws in $workspaces) {
    Write-Host "Reading $($ws.Name) ..."
    Push-Location $ws.Path
    try {
        $json = & cargo metadata --format-version 1 --all-features 2>$null
        if (-not $json) { throw "cargo metadata produced nothing in $($ws.Path)" }
        $meta = $json | ConvertFrom-Json
    } finally {
        Pop-Location
    }

    foreach ($p in $meta.packages) {
        # The workspace's own crates are the subject of this file, not entries in it.
        if ($p.name -like 'pmca-*') { continue }
        $key = "$($p.name) $($p.version)"
        if ($packages.Contains($key)) { continue }

        $packages[$key] = [pscustomobject]@{
            Name       = $p.name
            Version    = $p.version
            License    = if ($p.license) { $p.license } else { '(see licence file)' }
            Repository = $p.repository
            # manifest_path points into the unpacked registry source, which is
            # where the crate's own LICENSE files sit.
            Dir        = Split-Path -Parent $p.manifest_path
            Notices    = @()
        }
    }
}

Write-Host "Collecting copyright notices from $($packages.Count) packages ..."

$withNotice = 0
foreach ($pkg in $packages.Values) {
    if (-not (Test-Path $pkg.Dir)) { continue }

    $files = Get-ChildItem -Path $pkg.Dir -File -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -match '^(LICEN[SC]E|COPYING|NOTICE|UNLICENSE)' }

    $lines = @()
    foreach ($f in $files) {
        # 400 lines is past the header of every common licence; the notice is
        # never further down than that, and Apache-2.0 is long.
        $head = Get-Content -Path $f.FullName -TotalCount 400 -ErrorAction SilentlyContinue
        foreach ($line in $head) {
            $t = $line.Trim()

            # -cmatch, not -match: PowerShell compares case-insensitively by
            # default, and the body of the Apache-2.0 text is full of sentences
            # containing the word "copyright" in lower case ("...copyright
            # notice that is included in or attached to the work..."). Those are
            # prose, not notices, and an earlier version of this script
            # collected them as though they were.
            # `Copyright` or `©` only. A bare `(c)` at the start of a line is
            # not a notice in practice: the Apache-2.0 text numbers its
            # redistribution conditions (a), (b), (c), (d), and the third of
            # them was landing in this file as though Microsoft had written it.
            if ($t -cnotmatch '^(Copyright|©)') { continue }

            # Unfilled templates are not notices. A year is deliberately *not*
            # required: Microsoft's crates say "Copyright (c) Microsoft
            # Corporation." with no year at all, and demanding one dropped
            # every `windows-*` package from this file. Note the braces —
            # Apache-2.0 writes `{yyyy}`, MIT templates `[yyyy]`.
            if ($t -match 'yyyy|<year>|name of copyright owner|<COPYRIGHT HOLDER>|\[fullname\]|\[name\]') { continue }

            # "Copyright" alone, with nothing after it, is a heading.
            if ($t -cmatch '^Copyright\s*$') { continue }

            if ($lines -notcontains $t) { $lines += $t }
        }
    }
    $pkg.Notices = $lines
    if ($lines.Count -gt 0) { $withNotice++ }
}

$sorted = $packages.Values | Sort-Object Name, Version
$licenseSummary = $sorted | Group-Object License | Sort-Object Count -Descending

$sb = [System.Text.StringBuilder]::new()
$null = $sb.AppendLine('# Third-party licences')
$null = $sb.AppendLine()
$null = $sb.AppendLine('This file is generated automatically from the dependency graph of this')
$null = $sb.AppendLine('project; it is not written by hand.')
$null = $sb.AppendLine()
$null = $sb.AppendLine('The binaries this project distributes are statically linked and therefore')
$null = $sb.AppendLine('contain the code of the packages below. MIT and Apache-2.0 both require their')
$null = $sb.AppendLine('copyright notices to be reproduced with such a distribution, which is what')
$null = $sb.AppendLine('this file is for. It ships alongside `LICENSE.txt` in every release archive.')
$null = $sb.AppendLine()
$null = $sb.AppendLine('The licence of this project itself is in [LICENSE.txt](LICENSE.txt) and is not')
$null = $sb.AppendLine('repeated here.')
$null = $sb.AppendLine()
$null = $sb.AppendLine("Generated: $(Get-Date -Format 'yyyy-MM-dd')")
$null = $sb.AppendLine()
$null = $sb.AppendLine("* Packages: **$($sorted.Count)**")
$null = $sb.AppendLine("* With a copyright notice recovered from their sources: **$withNotice**")
$null = $sb.AppendLine("* Workspaces read: $($workspaces.Name -join ', ')")
$null = $sb.AppendLine()
$null = $sb.AppendLine('## Licences in use')
$null = $sb.AppendLine()
$null = $sb.AppendLine('| Licence (SPDX) | Packages |')
$null = $sb.AppendLine('| --- | ---: |')
foreach ($g in $licenseSummary) {
    $null = $sb.AppendLine("| ``$($g.Name)`` | $($g.Count) |")
}
$null = $sb.AppendLine()
$null = $sb.AppendLine('A slash or `OR` means the package offers a choice; this project relies on the')
$null = $sb.AppendLine('permissive option in every such case. The only copyleft licence present is')
$null = $sb.AppendLine('MPL-2.0, which is file-based: it obliges anyone who modifies those files to')
$null = $sb.AppendLine('publish the changes, and reaches no further. None of them is modified here.')
$null = $sb.AppendLine('One obligation applies even without changes: whoever distributes MPL-2.0 code')
$null = $sb.AppendLine('in executable form must tell the recipients where its source is available')
$null = $sb.AppendLine('(MPL-2.0 section 3.2). The `Source` link under each package below does that.')
$null = $sb.AppendLine()
$null = $sb.AppendLine('## Packages')
$null = $sb.AppendLine()

foreach ($p in $sorted) {
    $null = $sb.AppendLine("### $($p.Name) $($p.Version)")
    $null = $sb.AppendLine()
    $null = $sb.AppendLine("* Licence: ``$($p.License)``")
    if ($p.Repository) { $null = $sb.AppendLine("* Source: $($p.Repository)") }
    if ($p.Notices.Count -gt 0) {
        $null = $sb.AppendLine()
        foreach ($n in $p.Notices) { $null = $sb.AppendLine("  > $n") }
    }
    $null = $sb.AppendLine()
}

# UTF-8 without BOM: the file is read by tools as much as by people.
[System.IO.File]::WriteAllText($OutFile, $sb.ToString(), (New-Object System.Text.UTF8Encoding $false))
Write-Host "Wrote $OutFile ($($sorted.Count) packages, $withNotice with notices)."
