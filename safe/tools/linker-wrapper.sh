#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
SAFE_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
OUT=''
PREV=''

for ARG in "$@"; do
  if [ "$PREV" = "-o" ]; then
    OUT="$ARG"
    break
  fi
  PREV="$ARG"
done

cc "$@"

if [ -z "$OUT" ] || [ ! -f "$OUT" ]; then
  exit 0
fi

case "$(basename "$OUT")" in
  libsharpyuv.so|libwebp.so|libwebpdecoder.so|libwebpdemux.so|libwebpmux.so)
    LIB_NAME=$(basename "$OUT" .so)
    EXPORTS_PATH="$SAFE_DIR/abi/original/$LIB_NAME.exports"
    if [ ! -f "$EXPORTS_PATH" ]; then
      exit 0
    fi
    HELPER_SOURCE="$SAFE_DIR/tools/elf_prune.rs"
    HELPER_BINARY="$SAFE_DIR/target/tools/elf_prune"
    mkdir -p "$(dirname "$HELPER_BINARY")"
    if [ ! -x "$HELPER_BINARY" ] || [ "$HELPER_SOURCE" -nt "$HELPER_BINARY" ]; then
      rustc "$HELPER_SOURCE" -O -o "$HELPER_BINARY"
    fi
    "$HELPER_BINARY" hide "$OUT" "$EXPORTS_PATH"
    ;;
esac
