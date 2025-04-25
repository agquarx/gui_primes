#!/bin/bash

# Function to remove block comments using perl
remove_block_comments() {
    local file=$1
    
    # Use perl to remove block comments
    perl -0777 -i -pe 's{/\*.*?\*/}{}gs' "$file"
    
    # Remove empty lines
    sed -i '' '/^[[:space:]]*$/d' "$file"
    
    echo "Removed block comments from $file"
}

# Process all Rust files except those in target directory
find src -name "*.rs" -not -path "*/target/*" | while read file; do
    remove_block_comments "$file"
done

echo "Block comments removed from all files."
