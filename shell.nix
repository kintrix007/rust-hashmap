let
  url = "https://github.com/NixOS/nixpkgs/archive/6998cf86e9a6ef83b32956337f65aba8656671fe.tar.gz";
in
{ pkgs ? import (fetchTarball url) { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer
  ];
}
