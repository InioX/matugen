{
  description = " A material you color generation tool for linux ";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
      pkgs = nixpkgs.legacyPackages;
    in
    {
      packages = eachSystem (system: {
        default = pkgs.${system}.callPackage ./. { };
      });

      devShells = eachSystem (system: {
        default = pkgs.${system}.callPackage ./shell.nix { };

        # For testing with the ts script
        node = pkgs.${system}.mkShell {
          buildInputs = with pkgs.${system}; [
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
