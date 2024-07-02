self:
{
  config,
  pkgs,
  lib,
  ...
}:
let
  cfg = config.programs.matugen;
in
{
  options.programs.matugen = {
    enable = lib.mkEnableOption "matugen";
    package = lib.mkPackageOption pkgs "matugen" { };
    settings = lib.mkOption { type = lib.types.attrs; };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
    xdg.configFile."matugen/config.toml".source = lib.mkIf (cfg.settings != null) (
      (pkgs.formats.toml { }).generate "matugen" cfg.settings
    );
  };
}
