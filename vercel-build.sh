#!/bin/bash
set -e

echo "=== Memulai Build Rust WebAssembly untuk Vercel ==="

# 1. Pastikan Rust & Cargo tersedia
if ! command -v rustc &> /dev/null; then
    echo "Mengunduh Rust Toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    source "$HOME/.cargo/env"
fi

# 2. Tambahkan target wasm32
echo "Menambahkan target wasm32-unknown-unknown..."
rustup target add wasm32-unknown-unknown

# 3. Unduh biner Trunk jika belum tersedia
if ! command -v trunk &> /dev/null; then
    echo "Memasang Trunk v0.21.14..."
    mkdir -p "$HOME/.cargo/bin"
    curl -L -s https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C "$HOME/.cargo/bin"
    export PATH="$HOME/.cargo/bin:$PATH"
fi

# 4. Bangun produksi biner WebAssembly
echo "Menjalankan trunk build --release..."
trunk build --release

echo "=== Build Sukses! Artefak siap di folder dist/ ==="
