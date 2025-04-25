#!/bin/bash

# Function to remove comments from a Rust file
remove_comments() {
    local file=$1
    
    # Create a backup
    cp "$file" "${file}.withcomments"
    
    # Remove line comments and doc comments (//...)
    sed -i '' 's|//.*$||g' "$file"
    
    # Remove block comments (/* ... */)
    # This is a simplistic approach, might not work for nested comments or comments in strings
    # For a proper approach, a dedicated Rust parser would be needed
    sed -i '' ':a;N;$!ba;s|/\*\_.\*\*/||g' "$file"
    
    # Remove empty lines
    sed -i '' '/^[[:space:]]*$/d' "$file"
    
    echo "Removed comments from $file"
}

# Process all Rust files except those in target directory
find src -name "*.rs" -not -path "*/target/*" | while read file; do
    remove_comments "$file"
done

echo "Comments removed from all files. Original files saved with .withcomments extension."
