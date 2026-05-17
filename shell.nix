{}: let
  rustOverlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> {overlays = [rustOverlay];};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      cargo
      rustc
      rustfmt
      crate2nix
      rust-analyzer
    ];
    shellHook = ''
      echo "Welcome to Rust dev shell"
      rustc --version
      cargo -V
    '';
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  }
