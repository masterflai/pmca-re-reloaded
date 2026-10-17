# Generates the application icons for the Tauri bundle.
#
# Run once (or after changing the artwork) from the repository root:
#   powershell -File crates/pmca-gui/src-tauri/icons/generate-icons.ps1
#
# The icons are committed, so a normal build does not need this script. It
# exists so the artwork stays reproducible instead of being an opaque binary.

Add-Type -AssemblyName System.Drawing

$here = Split-Path -Parent $MyInvocation.MyCommand.Path

# Fluent-style accent used by the GUI stylesheet.
$accent = [System.Drawing.Color]::FromArgb(255, 0, 120, 212)
$body = [System.Drawing.Color]::FromArgb(255, 32, 32, 32)
$glass = [System.Drawing.Color]::FromArgb(255, 240, 240, 240)

function Draw-Camera($g, [single]$x, [single]$y, [single]$size) {
    $u = $size / 32.0
    # Camera body.
    $bodyRect = New-Object System.Drawing.RectangleF(($x + $u * 2), ($y + $u * 7), ($u * 28), ($u * 19))
    $bodyBrush = New-Object System.Drawing.SolidBrush($body)
    $g.FillRectangle($bodyBrush, $bodyRect)
    # Viewfinder hump.
    $humpRect = New-Object System.Drawing.RectangleF(($x + $u * 11), ($y + $u * 4), ($u * 10), ($u * 4))
    $g.FillRectangle($bodyBrush, $humpRect)
    # Lens barrel.
    $lensRect = New-Object System.Drawing.RectangleF(($x + $u * 10), ($y + $u * 11), ($u * 12), ($u * 12))
    $accentBrush = New-Object System.Drawing.SolidBrush($accent)
    $g.FillEllipse($accentBrush, $lensRect)
    # Lens glass.
    $glassRect = New-Object System.Drawing.RectangleF(($x + $u * 13), ($y + $u * 14), ($u * 6), ($u * 6))
    $glassBrush = New-Object System.Drawing.SolidBrush($glass)
    $g.FillEllipse($glassBrush, $glassRect)
    $bodyBrush.Dispose(); $accentBrush.Dispose(); $glassBrush.Dispose()
}

function New-IconBitmap([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.Clear([System.Drawing.Color]::Transparent)
    Draw-Camera $g 0 0 $size
    $g.Dispose()
    return $bmp
}

# PNG sizes expected by the Tauri bundler on Linux/macOS.
foreach ($size in 32, 128, 256, 512) {
    $bmp = New-IconBitmap $size
    $name = if ($size -eq 256) { "128x128@2x.png" } elseif ($size -eq 512) { "icon.png" } else { "${size}x${size}.png" }
    $bmp.Save((Join-Path $here $name), [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
}

# Multi-resolution .ico (required by tauri-build on Windows). Each entry is a
# PNG blob, which the Windows resource compiler accepts for Vista and newer.
function Write-Ico([string]$path, [scriptblock]$render) {
    $sizes = 16, 32, 48, 64, 256
    $blobs = @()
    foreach ($size in $sizes) {
        $bmp = & $render $size
        $ms = New-Object System.IO.MemoryStream
        $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
        $blobs += , $ms.ToArray()
        $ms.Dispose(); $bmp.Dispose()
    }

    $out = New-Object System.IO.MemoryStream
    $w = New-Object System.IO.BinaryWriter($out)
    $w.Write([uint16]0)                # reserved
    $w.Write([uint16]1)                # type: icon
    $w.Write([uint16]$sizes.Count)
    $offset = 6 + 16 * $sizes.Count
    for ($i = 0; $i -lt $sizes.Count; $i++) {
        $s = $sizes[$i]
        $w.Write([byte]($(if ($s -ge 256) { 0 } else { $s })))  # width (0 == 256)
        $w.Write([byte]($(if ($s -ge 256) { 0 } else { $s })))  # height
        $w.Write([byte]0)              # palette entries
        $w.Write([byte]0)              # reserved
        $w.Write([uint16]1)            # color planes
        $w.Write([uint16]32)           # bits per pixel
        $w.Write([uint32]$blobs[$i].Length)
        $w.Write([uint32]$offset)
        $offset += $blobs[$i].Length
    }
    foreach ($blob in $blobs) { $w.Write($blob) }
    $w.Flush()
    [System.IO.File]::WriteAllBytes($path, $out.ToArray())
    $w.Dispose(); $out.Dispose()
}

Write-Ico (Join-Path $here "icon.ico") { param($size) New-IconBitmap $size }

# Multi-resolution .icns for the macOS bundle. The bundler can derive one from
# the PNGs, but that derivation is a step we cannot verify from here, so the
# file is built explicitly and committed like the .ico.
#
# Every entry is a PNG blob, which macOS accepts since 10.7. The type codes are
# fixed names for a pixel size: `ic11` is 16pt at 2x, hence 32 pixels.
$icnsTypes = [ordered]@{
    "ic11" = 32; "ic12" = 64; "ic07" = 128; "ic13" = 256
    "ic08" = 256; "ic14" = 512; "ic09" = 512; "ic10" = 1024
}

# ICNS is big-endian, and BinaryWriter is not.
function Write-BigEndianUInt32([System.IO.Stream]$stream, [uint32]$value) {
    $bytes = [System.BitConverter]::GetBytes($value)
    if ([System.BitConverter]::IsLittleEndian) { [array]::Reverse($bytes) }
    $stream.Write($bytes, 0, 4)
}

$entries = @()
foreach ($type in $icnsTypes.Keys) {
    $bmp = New-IconBitmap $icnsTypes[$type]
    $ms = New-Object System.IO.MemoryStream
    $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
    $entries += , @{ Type = $type; Data = $ms.ToArray() }
    $ms.Dispose(); $bmp.Dispose()
}

$icns = New-Object System.IO.MemoryStream
$ascii = [System.Text.Encoding]::ASCII
$magic = $ascii.GetBytes("icns")
$icns.Write($magic, 0, 4)
# Total size counts the 8-byte header plus every entry including its own header.
$total = 8
foreach ($entry in $entries) { $total += 8 + $entry.Data.Length }
Write-BigEndianUInt32 $icns ([uint32]$total)
foreach ($entry in $entries) {
    $typeBytes = $ascii.GetBytes($entry.Type)
    $icns.Write($typeBytes, 0, 4)
    Write-BigEndianUInt32 $icns ([uint32](8 + $entry.Data.Length))
    $icns.Write($entry.Data, 0, $entry.Data.Length)
}
[System.IO.File]::WriteAllBytes((Join-Path $here "icon.icns"), $icns.ToArray())
$icns.Dispose()

# ---------------------------------------------------------------------------
# Windows installer artwork (NSIS and WiX), in the look of the app and the
# social preview: light surface, dark wordmark, accent "RELOADED".

$installerDir = Join-Path (Split-Path -Parent $here) "windows"
$brandAccent = [System.Drawing.Color]::FromArgb(255, 0, 103, 192)  # --accent in styles.css
$ink = [System.Drawing.Color]::FromArgb(255, 27, 27, 27)
$muted = [System.Drawing.Color]::FromArgb(255, 92, 92, 92)

# The camera, moved up-left to make room for a download badge.
function New-InstallerIconBitmap([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.Clear([System.Drawing.Color]::Transparent)
    Draw-Camera $g 0 0 ($size * 0.84)

    $u = $size / 32.0
    $white = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::White)
    $badge = New-Object System.Drawing.SolidBrush($accent)
    $g.FillEllipse($white, ($u * 16.5), ($u * 16.5), ($u * 15.5), ($u * 15.5))
    $g.FillEllipse($badge, ($u * 18), ($u * 18), ($u * 12.5), ($u * 12.5))
    $g.FillRectangle($white, ($u * 23.1), ($u * 20.5), ($u * 2.3), ($u * 4.6))
    $arrow = [System.Drawing.PointF[]]@(
        (New-Object System.Drawing.PointF(($u * 20.3), ($u * 24.6))),
        (New-Object System.Drawing.PointF(($u * 28.2), ($u * 24.6))),
        (New-Object System.Drawing.PointF(($u * 24.25), ($u * 28.6)))
    )
    $g.FillPolygon($white, $arrow)
    $white.Dispose(); $badge.Dispose(); $g.Dispose()
    return $bmp
}

Write-Ico (Join-Path $installerDir "installer.ico") { param($size) New-InstallerIconBitmap $size }

function New-Canvas([int]$width, [int]$height, [single]$scale = 1) {
    $bmp = New-Object System.Drawing.Bitmap([int]($width * $scale), [int]($height * $scale), [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit
    $g.Clear([System.Drawing.Color]::White)
    $g.ScaleTransform($scale, $scale)
    return $bmp, $g
}

# NSIS and WiX only take uncompressed 24-bit bitmaps.
function Save-Bmp($bmp, [string]$name) {
    $rect = New-Object System.Drawing.Rectangle(0, 0, $bmp.Width, $bmp.Height)
    $flat = $bmp.Clone($rect, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
    $flat.Save((Join-Path $installerDir $name), [System.Drawing.Imaging.ImageFormat]::Bmp)
    $flat.Dispose()
}

function Draw-Wordmark($g, [single]$x, [single]$y, [single]$px) {
    $font = New-Object System.Drawing.Font("Segoe UI", $px, [System.Drawing.FontStyle]::Bold, [System.Drawing.GraphicsUnit]::Pixel)
    $inkBrush = New-Object System.Drawing.SolidBrush($ink)
    $accentBrush = New-Object System.Drawing.SolidBrush($brandAccent)
    $g.DrawString("PMCA-RE", $font, $inkBrush, $x, $y)
    $g.DrawString("RELOADED", $font, $accentBrush, $x, ($y + $px * 1.12))
    $font.Dispose(); $inkBrush.Dispose(); $accentBrush.Dispose()
}

# Welcome/finish panel: the social preview in portrait, with a faint lens ring.
function Draw-Panel($g, [int]$width, [int]$height) {
    $area = New-Object System.Drawing.Rectangle(0, 0, $width, $height)
    $g.SetClip($area)
    $top = [System.Drawing.Color]::FromArgb(255, 248, 250, 253)
    $bottom = [System.Drawing.Color]::FromArgb(255, 226, 236, 248)
    $fill = New-Object System.Drawing.Drawing2D.LinearGradientBrush($area, $top, $bottom, 90.0)
    $g.FillRectangle($fill, $area)
    $fill.Dispose()

    $ring = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(28, $brandAccent), 20)
    $g.DrawEllipse($ring, ($width - 70), ($height - 110), 170, 170)
    $ring.Width = 8
    $g.DrawEllipse($ring, ($width - 25), ($height - 65), 80, 80)
    $ring.Dispose()

    Draw-Camera $g 18 30 56
    Draw-Wordmark $g 16 100 24
    $font = New-Object System.Drawing.Font("Segoe UI", 12, [System.Drawing.FontStyle]::Regular, [System.Drawing.GraphicsUnit]::Pixel)
    $brush = New-Object System.Drawing.SolidBrush($muted)
    $g.DrawString("Camera Toolkit", $font, $brush, 18, 160)
    $font.Dispose(); $brush.Dispose()
}

# NSIS welcome/finish page, 164x314 logical. MUI stretches the bitmap to the
# control, so it is drawn at 2x to stay sharp on scaled displays.
$bmp, $g = New-Canvas 164 314 2
Draw-Panel $g 164 314
$g.Dispose(); Save-Bmp $bmp "nsis-sidebar.bmp"; $bmp.Dispose()

# NSIS page header, 150x57 logical; MUI draws it at the left, on a white header.
$bmp, $g = New-Canvas 150 57 2
Draw-Camera $g 10 8 40
Draw-Wordmark $g 56 13 13
$g.Dispose(); Save-Bmp $bmp "nsis-header.bmp"; $bmp.Dispose()

# WiX welcome/exit dialog, 493x312 logical; WiX puts its text right of x=180.
$bmp, $g = New-Canvas 493 312 2
Draw-Panel $g 164 312
$g.Dispose(); Save-Bmp $bmp "wix-dialog.bmp"; $bmp.Dispose()

# WiX page banner, 493x58 logical; WiX puts its text at the left.
$bmp, $g = New-Canvas 493 58 2
Draw-Camera $g 441 9 40
$g.Dispose(); Save-Bmp $bmp "wix-banner.bmp"; $bmp.Dispose()

Write-Output "icons written to $here"
