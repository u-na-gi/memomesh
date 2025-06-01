FROM ghcr.io/u-na-gi/rust-node-deno-wasm-pack:latest

SHELL ["/bin/bash", "-c"]

RUN apt update -y && \
  apt upgrade -y

RUN apt install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  curl \
  git \
  protobuf-compiler


RUN cargo install worker-build

#rustのtargetのパスを変更
RUN mkdir -p /.rust/target
ENV CARGO_TARGET_DIR=/.rust/target
WORKDIR /workspace
COPY . .

# rust
RUN cargo build
WORKDIR /

RUN npm install -g pnpm

COPY develop/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
