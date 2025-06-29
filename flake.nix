{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    fenix.url = "github:nix-community/fenix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem = { pkgs, lib, system, ... }:
        let
          rustToolchain = with inputs.fenix.packages.${system}; combine [
            stable.cargo
            stable.clippy
            stable.rust-src
            stable.rustc
            latest.rustfmt
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
          rustBuildInputs = [
            pkgs.openssl
            pkgs.libiconv
            pkgs.pkg-config
          ] ++ lib.optionals pkgs.stdenv.isLinux [
            pkgs.glib
            pkgs.gtk3
            pkgs.libsoup_3
            pkgs.webkitgtk_4_1
            pkgs.xdotool
          ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
            IOKit
            Carbon
            WebKit
            Security
            Cocoa
          ]);
        in
        {
          devShells.default = pkgs.mkShell {
            name = "dioxus-dev";
            buildInputs = rustBuildInputs;
            nativeBuildInputs = [
              rustToolchain
              inputs.fenix.packages.${system}.rust-analyzer
              pkgs.wasm-bindgen-cli
            ];
            shellHook = ''
              # For rust-analyzer 'hover' tooltips to work.
              export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
            '';
          };
        };
    };
}
