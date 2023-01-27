#!/bin/bash

image_size="400x200"

for image_file in $(find images -name "*.jpg");
do 
    if [[ $image_file != *"preview"* ]]; then
        echo "generating preview for $image_file"
        convert -resize $image_size! "$image_file" -quality 100% "${image_file%.*}_preview.jpg"; 
    fi
done;
