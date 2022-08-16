{ pkgs ? import (
  builtins.fetchTarball {
    url = "https://github.com/nixos/nixpkgs/archive/21.11.tar.gz";
    sha256 = "162dywda2dvfj1248afxc45kcrg83appjd0nmdb541hl7rnncf02";
  }
) {} }:

with pkgs;

mkShell {
  buildInputs = [
    rustup
    cargo
    cargo-cross
    docker
  ];

  shellHook = ''
    rustup toolchain install stable
  '';
}
