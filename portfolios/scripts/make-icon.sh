#!/usr/bin/env bash
# make-icon.sh — Convert an image to a Win95-style 48x48 dithered icon
#
# Usage:
#   ./scripts/make-icon.sh <input> <output.png>
#
# Requirements:
#   ImageMagick (convert command)
#
# What it does:
#   1. Resizes the input image to exactly 48x48 pixels (forcing aspect ratio)
#   2. Reduces to 256 colors using Floyd-Steinberg dithering for an authentic
#      retro palette look, similar to Windows 95 .ico files
#   3. Writes the result as a PNG

set -euo pipefail

# --- Usage -------------------------------------------------------------------

if [[ $# -ne 2 ]]; then
    echo "Usage: $(basename "$0") <input> <output.png>" >&2
    echo "" >&2
    echo "Converts an image to a 48x48, 256-color, Floyd-Steinberg dithered" >&2
    echo "PNG suitable for use as a Windows 95-style icon." >&2
    exit 1
fi

INPUT="$1"
OUTPUT="$2"

# --- Validate input ----------------------------------------------------------

if [[ ! -f "$INPUT" ]]; then
    echo "Error: input file not found: $INPUT" >&2
    exit 1
fi

if ! command -v convert &>/dev/null; then
    echo "Error: ImageMagick 'convert' command not found. Install ImageMagick." >&2
    exit 1
fi

# --- Convert -----------------------------------------------------------------

# Pipeline explanation:
#   -resize 48x48!        Force exact 48×48 (ignores aspect ratio, like real icons)
#   -dither FloydSteinberg Use error-diffusion dithering for a classic pixelated look
#   -remap wizard:        Remap to ImageMagick's built-in wizard palette (256 Web-safe
#                         colors), which approximates the old Windows system palette
#   -type Palette         Store as a palette (indexed) PNG — keeps file size tiny and
#                         reinforces the retro aesthetic
convert "$INPUT" \
    -resize 48x48! \
    -dither FloydSteinberg \
    -remap wizard: \
    -type Palette \
    "$OUTPUT"

echo "Icon written to $OUTPUT (48x48, 256-color palette)"
