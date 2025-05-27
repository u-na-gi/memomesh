#!/bin/bash


find . -name '*.proto' | xargs npx protoc --plugin=protoc-gen-ts=./node_modules/.bin/protoc-gen-ts_proto \
  --ts_out=../backend/scenario-test/types \
  --proto_path=.