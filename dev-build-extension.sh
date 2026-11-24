#!/usr/bin/env bash
set -euo pipefail

cd manglua-ls
cargo build

cd ../manglua-ls-vscode
bun compile

cp ../README.md .
mkdir -p server
cp ../manglua-ls/target/debug/manglua-ls ./server
chmod +x server/manglua-ls

bunx @vscode/vsce package --target linux-x64
rm README.md
mv *.vsix ..
