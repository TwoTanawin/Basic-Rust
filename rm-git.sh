#!/bin/bash

# List of directories
dirs=(
    "channels" "enums" "hello-crabby" "loops" "rust-threads" "structs" "vectors"
    "error-handling" "immutable-vs-mutable" "macros" "smart-pointer" "tokio-async"
    "control-flow" "function" "iterators-and-closures" "modules-and-crates" 
    "string-vs-borrow-string" "traits" "data-types" "hashMap" "lifetime"
    "ownership-and-borrowing" "traits-as-a-types"
)

# Loop through directories and remove .git
for dir in "${dirs[@]}"; do
    if [ -d "$dir/.git" ]; then
        rm -rf "$dir/.git"
        echo "Removed .git from $dir"
    fi
done
