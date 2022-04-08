#!/bin/bash
for filename in src/pictures/*.mscgen; do
    ./mscgen-0.20/bin/mscgen -T eps -o "src/pictures/$(basename "$filename" .mscgen).eps" -i "$filename"
    ./mscgen-0.20/bin/mscgen -T png -o "src/pictures/$(basename "$filename" .mscgen).png" -i "$filename"
done
