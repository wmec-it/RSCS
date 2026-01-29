#!/usr/bin/bash

#:& Formats code with default formatter
cargo fmt

#:% Not implemented (already made TODO)
bash ./lib/json_formatting.sh

bash ./lib/convert_endings.sh