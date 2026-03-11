{
  description = "nrf-aes-gcm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libPath =
          with pkgs;
          lib.makeLibraryPath [
            llvmPackages.libclang.lib
          ];

        rust = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "llvm-tools-preview" ];
          targets = [ "thumbv7em-none-eabihf" "thumbv7em-none-eabi" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
          ];
          
          packages = with pkgs; [
            rust
            gdb
            usbutils
            clang_20
            llvmPackages_20.bintools
            cargo-binutils
          ];

          LD_LIBRARY_PATH = libPath;
        };
      }
    );
}
