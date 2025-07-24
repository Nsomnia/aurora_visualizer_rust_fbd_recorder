#!/bin/bash
# Simple commit script for the project

echo "Adding all changes..."
git add .

echo "Enter commit message:"
read commit_message

if [ -z "$commit_message" ]; then
    echo "No commit message provided. Using default message."
    git commit -m "Updated project files"
else
    git commit -m "$commit_message"
fi

echo "Pushing to remote repository..."
git push origin main

echo "Done!"
