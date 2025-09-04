{
  description = " A material you color generation tool for linux ";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };
  outputs = {
    self,
    nixpkgs,
    systems,
  }: let
    forAllSystems = nixpkgs.lib.genAttrs (import systems);
    pkgsFor = nixpkgs.legacyPackages;
  in {
    packages = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./. {};
    });
    devShells = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./shell.nix {};

      # For testing with the ts script
      node = pkgsFor.${system}.mkShell {
        buildInputs = with pkgsFor.${system}; [
          nodejs_20
          nodePackages.pnpm
        ];
      };
    });
    nixosModules = {
      matugen = import ./module.nix self;
      default = self.nixosModules.matugen;
    };
  };
}
