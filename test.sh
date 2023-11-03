#!/bin/zsh

# Clear playlist
cargo run -q -- debug clear-playlist 6WQLiVCBXBT92e7WHRdY0u

# Recreate playlist
cargo run -q -- copy 67ETB0zzB5QGzNaLW2I7Qs into 6WQLiVCBXBT92e7WHRdY0u

# Sort playlist
cargo run -q -- sort 6WQLiVCBXBT92e7WHRdY0u album-release track-number
