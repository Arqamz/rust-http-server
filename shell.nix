{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc      # Rust compiler
    pkgs.cargo      # Rust package manager and build tool
    pkgs.rustfmt    # Rust code formatter
    pkgs.clippy     # Rust linter
  ];

  shellHook = ''
    export CARGO_HOME=$PWD/.cargo
    export RUSTUP_HOME=$PWD/.rustup
  '';
}
