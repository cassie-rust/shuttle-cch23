#!/bin/bash

# Specify the directory where you want to create the files
directory="."

# Loop from day1 to day24 and create files
for day in {1..24}; do
    filename="$directory/day${day}.rs"
    touch "$filename"
    echo "use axum::Router;" > "$filename"
    echo "" >> "$filename"
    echo "pub fn router() -> Router { Router::new() }" >> "$filename"
done

echo "Files created successfully in $directory."

cargo fmt