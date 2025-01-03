#!/bin/bash

# List of directories
dirs=(
    "channels"
    "control-flow"
    "data-types"
    "enums"
    "error-handling"
    "function"
    "hashMap"
    "hello-crabby"
    "immutable-vs-mutable"
    "iterators-and-closures"
    "lifetime"
    "loops"
    "macros"
    "modules-and-crates"
    "ownership-and-borrowing"
    "rust-threads"
    "smart-pointer"
    "string-vs-borrow-string"
    "structs"
    "tokio-async"
    "traits"
    "traits-as-a-types"
    "vectors"
)

# Loop through each directory
for dir in "${dirs[@]}"; do
    cd "$dir"
    git add .
    git commit -m "Initial commit for $dir"
    cd ..
done
