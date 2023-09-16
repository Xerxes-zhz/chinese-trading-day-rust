{pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # rust 
    rustc
    cargo 
    rustfmt
    rust-analyzer
    clippy
    openssl
    pkgconfig
    
  ];
  RUST_BACKTRACE = 1;
}
