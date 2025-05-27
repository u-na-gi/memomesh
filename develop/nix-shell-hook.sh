#!/bin/bash

set -e  # エラー時に即終了
set -x  # デバッグログ出力（任意）


rustup target add wasm32-unknown-unknown || true

if [ ! -d $LOCAL_INSTALL ]; then
  cargo install worker-build --root $LOCAL_INSTALL || true
  curl -fsSL https://deno.land/install.sh | DENO_INSTALL=$LOCAL_INSTALL bash || true
else
  echo "Skipping cargo install (already installed)"
fi


# node_modules が空なら install
if [ ! -d node_modules ] || [ -z "$(ls -A node_modules 2>/dev/null)" ]; then
  echo "Installing Node packages with pnpm..."
  pnpm install || true
else
  echo "Skipping pnpm install (node_modules already populated)"
fi

#protocのpathを通す
node ./develop/updateProtoc.js


