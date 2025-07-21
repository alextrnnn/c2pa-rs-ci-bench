#!/bin/bash

set -e

cargo build --release

BIN=../target/release/c2patool

INPUT_ASSET="sample/image.jpg"
INPUT_MANIFEST="sample/test.json"
OUTPUT_FILE="../target/tmp/tmp.jpg"

HYPERFINE_OUTPUT=$(hyperfine --warmup 150\
  "rm -f $OUTPUT_FILE && $BIN $INPUT_ASSET -m $INPUT_MANIFEST -o $OUTPUT_FILE")

README="README.md"

# Remove existing Performance section and everything below it
sed -i.bak '/^### Performance/,$d' "$README"
rm -f "${README}.bak"

# Append new performance section
{
  echo -e "\n### Performance\n"
  echo '```'
  echo "$HYPERFINE_OUTPUT"
  echo '```'
} >> "$README"
