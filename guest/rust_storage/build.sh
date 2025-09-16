#!/usr/bin/env bash
set -e
CARGO_TARGET_DIR=./target-storage RUSTFLAGS="-Ctarget-feature=+crt-static -Zexport-executable-symbols" cargo build --release --bin storage
#RUSTFLAGS="-Zexport-executable-symbols" cargo build --target x86_64-unknown-linux-musl --release --bin storage
storage_binary=./target-storage/release/storage
objcopy -w --extract-symbol --strip-symbol=!remote* --strip-symbol=* $storage_binary storage.syms
gcc -static -O2 symbol_offset.c -o symbol_offset
./symbol_offset storage.syms 0x44000000

rm -rf target/
RUSTFLAGS="-Zexport-executable-symbols -C link_arg=-Wl,--undefined=do_calculation,--just-symbols=$PWD/storage.syms" cargo build --release --bin main
