#!/usr/bin/env bash
set -e

cd $(dirname "$0")
DIRNAME="$PWD"

cd "$DIRNAME/public"

mkdir -p "$DIRNAME/tmp"
cd "$DIRNAME/tmp"

cp "$DIRNAME/public/assets/logo.svg" .

for size in 16 32 48 128 256; do
    inkscape --export-type=png --export-filename="logo_$size.png" -w ${size} -h ${size} logo.svg
done

magick convert logo_16.png logo_32.png logo_48.png logo_128.png logo_256.png -colors 256 favicon.ico

cp favicon.ico "$DIRNAME/public"

cd "$DIRNAME"
\rm -rf tmp

echo "favicon creation completed!"
