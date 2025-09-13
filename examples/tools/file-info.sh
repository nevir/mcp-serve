#!/bin/bash
# ---
# name: FileInfo
# title: File Information
# description: Get information about a file including size, permissions, and type.
# input:
#   template: '{{filepath}}'
#   schema:
#     type: object
#     properties:
#       filepath:
#         type: string
#         description: "Path to the file to analyze"
#     required: ["filepath"]
# output:
#   template: |-
#     Size: (?<size>\d+) bytes
#     Type: (?<type>.*)
#     Permissions: (?<permissions>.*)
#   schema:
#     type: object
#     properties:
#       size:
#         type: string
#         description: "File size in bytes"
#       type:
#         type: string
#         description: "File type"
#       permissions:
#         type: string
#         description: "File permissions"
# ---

if [ -f "$1" ]; then
    size=$(stat -c%s "$1" 2>/dev/null || stat -f%z "$1" 2>/dev/null || echo "unknown")
    filetype=$(file -b "$1" 2>/dev/null || echo "unknown")
    perms=$(stat -c%A "$1" 2>/dev/null || stat -f%Sp "$1" 2>/dev/null || echo "unknown")
    
    echo "Size: $size bytes"
    echo "Type: $filetype"
    echo "Permissions: $perms"
elif [ -d "$1" ]; then
    echo "Error: Path is a directory, not a file" >&2
    exit 1
else
    echo "Error: File not found: $1" >&2
    exit 1
fi