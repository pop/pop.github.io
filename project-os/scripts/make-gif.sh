#!/usr/bin/env bash
# make-gif.sh — Convert a video clip to a looping 90s-style GIF
#
# Usage: ./scripts/make-gif.sh INPUT OUTPUT
#   INPUT   Source video file (mp4, mov, webm, etc.)
#   OUTPUT  Destination GIF file
#
# Tweak these to taste:
FPS=12          # Frames per second — lower = choppier/more retro
MAX_WIDTH=480   # Max output width in pixels (aspect ratio preserved)
PALETTE=/tmp/make-gif-palette-$$.png

set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 INPUT OUTPUT" >&2
    exit 1
fi

INPUT="$1"
OUTPUT="$2"

if [[ ! -f "$INPUT" ]]; then
    echo "Error: input file not found: $INPUT" >&2
    exit 1
fi

echo "Pass 1: generating palette..."
ffmpeg -v warning -i "$INPUT" \
    -vf "fps=${FPS},scale=${MAX_WIDTH}:-1:flags=lanczos,palettegen=stats_mode=diff" \
    -y "$PALETTE"

echo "Pass 2: encoding GIF with dithering..."
ffmpeg -v warning -i "$INPUT" -i "$PALETTE" \
    -filter_complex "fps=${FPS},scale=${MAX_WIDTH}:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle" \
    -loop 0 \
    -y "$OUTPUT"

rm -f "$PALETTE"
echo "Done: $OUTPUT"
