#!/bin/bash
# Usage: ./scripts/fix_symbols.sh <library.a>

set -e

LIB_FILE="$1"
OLD_SYM="rust_eh_personality"
NEW_SYM="__custom_rust_eh"

if [ -z "$LIB_FILE" ] || [ ! -f "$LIB_FILE" ]; then
    echo "Usage: $0 <library.a>"
    exit 1
fi

OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux)
        if [[ "$ARCH" == "aarch64" ]] || [[ "$ARCH" == "arm64" ]]; then
            TOOL="aarch64-linux-gnu-objcopy"
            if ! command -v $TOOL &> /dev/null; then
                echo "Error: Install binutils-aarch64-linux-gnu"
                exit 1
            fi
        elif [[ "$ARCH" == "armv7l" ]] || [[ "$ARCH" == "arm" ]]; then
            TOOL="arm-linux-gnueabihf-objcopy"
            if ! command -v $TOOL &> /dev/null; then
                echo "Error: Install binutils-arm-linux-gnueabihf"
                exit 1
            fi
        else
            TOOL="objcopy"
        fi
        ;;
    Darwin)
        TOOL="llvm-objcopy"
        if ! command -v $TOOL &> /dev/null; then
            echo "Error: Install llvm (brew install llvm)"
            exit 1
        fi
        ;;
    *)
        echo "Error: Unsupported OS: $OS"
        exit 1
        ;;
esac

cp "$LIB_FILE" "${LIB_FILE}.backup"
$TOOL --redefine-sym "$OLD_SYM=$NEW_SYM" "$LIB_FILE" "${LIB_FILE}.tmp" 2>/dev/null || {
    $TOOL --strip-symbol="$OLD_SYM" "$LIB_FILE" "${LIB_FILE}.tmp"
}
mv "${LIB_FILE}.tmp" "$LIB_FILE"
ranlib "$LIB_FILE" 2>/dev/null || true

echo "Fixed: $LIB_FILE"
