#!/bin/bash
# Find and fix all libraries recursively

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SEARCH_DIR="${1:-.}"

find "$SEARCH_DIR" -type f \( -name "*.a" -o -name "*.lib" \) | while read -r lib; do
    if [[ "$lib" == *.a ]]; then
        "$SCRIPT_DIR/fix_symbols.sh" "$lib"
    elif [[ "$lib" == *.lib ]] && [[ "$OS" == "Windows_NT" ]]; then
        cmd /c "$SCRIPT_DIR\\fix_symbols.cmd" "$lib"
    fi
done
