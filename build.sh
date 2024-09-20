#!/usr/bin/env bash

# resources
#  - https://kmdouglass.github.io/posts/complex-data-types-and-the-rust-ffi/
#  - https://users.rust-lang.org/t/solved-statically-linking-rust-library-yields-undefined-references/53815/6

rm -rf gen
mkdir -p gen

cargo clean
cargo build --release

cbindgen --lang c -o gen/exp-ffi.h .

cargo rustc -- --print native-static-libs

cc -o target/main src/main.c -lexp_ffi -lssl -lcrypto -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc -Ltarget/release

ldd target/main

echo "Output:"
target/main