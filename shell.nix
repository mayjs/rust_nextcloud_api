with import <nixpkgs> {};

let
rustChannel = latest.rustChannels.stable;
in
pkgs.mkShell {
  buildInputs = [
    rustChannel.rust
    rustChannel.rust-src
    cargo
    openssl
    pkg-config
    rust-analyzer
  ];

  shellHook = ''
    export RUST_SRC_PATH="${rustChannel.rust-src}/lib/rustlib/src/rust/library"
  '';
}
 
