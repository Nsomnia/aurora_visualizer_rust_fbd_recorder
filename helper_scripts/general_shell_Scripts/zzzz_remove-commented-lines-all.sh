#!/bin/bash

# Check if a filename is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <script-file>" >&2
    exit 1
fi

# Check if the file exists
if [ ! -f "$1" ]; then
    echo "Error: File $1 does not exist." >&2
    exit 1
fi

# Remove lines that are comments (starting with optional whitespace followed by #)
grep -v '^[[:space:]]*#' "$1"