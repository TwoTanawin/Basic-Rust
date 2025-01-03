#!/bin/bash

# Force remove existing git setup
rm -rf .git

# Initialize new repository
git init

# Add and commit all files
git add .
git commit -m "Initial commit: Add all Rust learning projects"

# Set up remote and push
git branch -M main
git remote add origin https://github.com/TwoTanawin/Basic-Rust.git
git push -u origin main --force
