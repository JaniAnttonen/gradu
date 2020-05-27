#!/bin/bash
for filename in pictures/*.mscgen; do
    mscgen -T eps -o "pictures/$(basename "$filename" .mscgen).eps" -i "$filename"
    mscgen -T png -o "pictures/$(basename "$filename" .mscgen).png" -i "$filename"
done
