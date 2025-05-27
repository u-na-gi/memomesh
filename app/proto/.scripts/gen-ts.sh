#!/bin/bash

find . -name '*.proto' | xargs npx protoc --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts_proto \
  --ts_out=../frontend/src/lib/types \
  --proto_path=.