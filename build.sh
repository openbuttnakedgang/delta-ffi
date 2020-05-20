#!/bin/sh

cargo build

cbindgen --config cbindgen.toml --crate delta-ffi --output lib/delta_ffi.h

export LD_LIBRARY_PATH=./target/debug
gcc -Wall -std=c11 examples/main.c -o main.out -I lib/  -L${LD_LIBRARY_PATH} -ldelta_ffi

RESULT=$?
if [ $RESULT -eq 0 ]; then
  ./main.out
fi

