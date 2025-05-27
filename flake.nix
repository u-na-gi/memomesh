{
  description = "Dev shell with Rust, WebAssembly, Deno, and other tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cowsay
            lolcat
            rustup
            llvm
            clang
            nodejs_22
            python3
            wasm-pack
            cargo-binstall
            cargo-watch
            cargo-generate
            curl
            bash
            pnpm
            libiconv
            bzip2
            protobuf
          ];

          shellHook = ''
            export LOCAL_INSTALL="$PWD/local-bin"
            bash develop/nix-shell-hook.sh
            export PATH="$LOCAL_INSTALL:$PATH"
          '';
        };
      });
}
