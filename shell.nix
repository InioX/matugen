{pkgs ? import <nixpkgs> {}}:
let 
    generate-filter-docs = pkgs.writeShellScriptBin "generate-filter-docs" ''
      cargo run --features filter-docs -- color hex "#ffffff" --filter-docs-html
    '';
in
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  # Additional tooling
  buildInputs = with pkgs; [
    rust-analyzer # LSP Server
    rustfmt # Formatter
    clippy # Linter

    generate-filter-docs
  ];
}
