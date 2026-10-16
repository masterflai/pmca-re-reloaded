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

function New-IconBitmap([int]$size) {
    $bmp = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
    $g.Clear([System.Drawing.Color]::Transparent)

    $u = $size / 32.0
    # Camera body.
    $bodyRect = New-Object System.Drawing.RectangleF(($u * 2), ($u * 7), ($u * 28), ($u * 19))
    $bodyBrush = New-Object System.Drawing.SolidBrush($body)
    $g.FillRectangle($bodyBrush, $bodyRect)
    # Viewfinder hump.
    $humpRect = New-Object System.Drawing.RectangleF(($u * 11), ($u * 4), ($u * 10), ($u * 4))
    $g.FillRectangle($bodyBrush, $humpRect)
    # Lens barrel.
    $lensRect = New-Object System.Drawing.RectangleF(($u * 10), ($u * 11), ($u * 12), ($u * 12))
    $accentBrush = New-Object System.Drawing.SolidBrush($accent)
    $g.FillEllipse($accentBrush, $lensRect)
    # Lens glass.
    $glassRect = New-Object System.Drawing.RectangleF(($u * 13), ($u * 14), ($u * 6), ($u * 6))
    $glassBrush = New-Object System.Drawing.SolidBrush($glass)
    $g.FillEllipse($glassBrush, $glassRect)

    $bodyBrush.Dispose(); $accentBrush.Dispose(); $glassBrush.Dispose(); $g.Dispose()
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
$sizes = 16, 32, 48, 64, 256
$blobs = @()
foreach ($size in $sizes) {
    $bmp = New-IconBitmap $size
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
[System.IO.File]::WriteAllBytes((Join-Path $here "icon.ico"), $out.ToArray())
$w.Dispose(); $out.Dispose()

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

Write-Output "icons written to $here"
