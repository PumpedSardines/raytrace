{ pkgs ? import <nixpkgs> { } }:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  buildInputs = [
    pkgs.darwin.apple_sdk.frameworks.ApplicationServices
    pkgs.darwin.apple_sdk.frameworks.CoreVideo
    pkgs.darwin.apple_sdk.frameworks.AppKit
  ];
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
