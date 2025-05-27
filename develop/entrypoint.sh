#!/bin/bash

set -e  # エラー時に即終了
set -x  # デバッグログ出力（任意）

# 初期処理
echo "🔧 Running pnpm install..."
pnpm install

# 任意：ここに追加の初期化処理を入れてもOK
# 例: echo "Waiting for DB..." && sleep 5



# コンテナを維持するための無限ループ
echo "✅ Setup complete. Holding container open..."
tail -f /dev/null
