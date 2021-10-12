#!/bin/bash
for filename in src/pictures/*.mscgen; do
    mscgen -T eps -o "src/pictures/$(basename "$filename" .mscgen).eps" -i "$filename"
    mscgen -T png -o "src/pictures/$(basename "$filename" .mscgen).png" -i "$filename"
done
