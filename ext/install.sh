#!/bin/bash
set -e

echo "Checking for Clang installation..."

if ! command -v clang &>/dev/null; then
    echo "Clang not found. Installing LLVM, Clang, and libclang..."

    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v apt-get &>/dev/null; then
            echo "Detected Ubuntu/Debian"
            sudo apt-get update
            sudo apt-get install -y llvm-dev libclang-dev clang
        elif command -v pacman &>/dev/null; then
            echo "Detected Arch Linux"
            sudo pacman -Syu --noconfirm llvm clang
        elif command -v dnf &>/dev/null; then
            echo "Detected Fedora"
            sudo dnf install -y llvm-devel clang
        else
            echo "Unsupported Linux distribution. Please install LLVM & Clang manually"
            exit 1
        fi

    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Detected macOS"
        if ! command -v brew &>/dev/null; then
            echo "Homebrew not found. Installing Homebrew..."
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi
        brew update
        brew install llvm
    else
        echo "Unsupported OS : $OSTYPE"
        exit 1
    fi
else
    echo "Clang is already installed"
fi

if ! command -v cargo &> /dev/null; then
    echo "Cargo is not installed. Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"
else
    echo "Cargo is installed. Updating Rust toolchain..."
    rustup update
fi

echo "Cleaning and building the project in release mode..."
cargo clean
cargo build --release --lib
echo "Build complete !"

OS=$(uname)
if [[ "$OS" == "MINGW"* ]] || [[ "$OS" == "MSYS"* ]] || [[ "$OS" == "CYGWIN"* ]]; then
    EXT_SUFFIX="dll"
elif [[ "$OS" == "Darwin" ]]; then
    EXT_SUFFIX="dylib"
else
    EXT_SUFFIX="so"
fi
echo "Detected OS : $OS , using extension : $EXT_SUFFIX"

EXT_FILE=$(find / -type f -iname "libphptgcalls.${EXT_SUFFIX}" 2>/dev/null | head -n 1)
if [ -z "$EXT_FILE" ]; then
    echo "Error : No extension file found in target/release"
    exit 1
fi
echo "Found extension file : $EXT_FILE"

EXT_DIR=$(php-config --extension-dir 2>/dev/null)
if [ -z "$EXT_DIR" ]; then
    echo "Error : Could not determine PHP extension directory via php-config"
    exit 1
fi
echo "PHP extension directory : $EXT_DIR"

OUTPUT_FILE="$EXT_DIR/phptgcalls.$EXT_SUFFIX"

cp "$EXT_FILE" "$OUTPUT_FILE" || { echo "Failed to copy extension file"; exit 1; }

NTGCALLS_LIB=$(find / -type f -name "libntgcalls.${EXT_SUFFIX}" 2>/dev/null | head -n 1)
if [ -z "$NTGCALLS_LIB" ]; then
    echo "Error : libntgcalls.${EXT_SUFFIX} not found"
    exit 1
fi

cp "$NTGCALLS_LIB" "$EXT_DIR/"

PHP_INI=$(php --ini | awk -F': ' '/Loaded Configuration File/ {print $2}' | xargs)
if [ -z "$PHP_INI" ]; then
    echo "Error : Could not find php.ini file"
    exit 1
fi

if grep -q "^extension=phptgcalls.$EXT_SUFFIX" "$PHP_INI"; then
    echo "phptgcalls.$EXT_SUFFIX is already enabled in php.ini"
else
    echo "Adding extension=phptgcalls.$EXT_SUFFIX to php.ini"
    echo -e "\nextension=phptgcalls.$EXT_SUFFIX" >> "$PHP_INI"
    echo "phptgcalls.$EXT_SUFFIX has been added to php.ini"
fi

echo "Verifying PHP extension load..."

php -m | grep -i phptgcalls && echo "extension is loaded !" || echo "extension is not loaded. Check your configuration"

php -r "print('Version : '.PHP_TGCALLS_VERSION.PHP_EOL);"

echo "Installation successful !"