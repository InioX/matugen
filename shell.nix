{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {

  buildInputs = with pkgs; [
    python3
    poetry
    python310Packages.pip
  ];

}