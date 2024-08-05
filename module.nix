# this arg is the matugen flake input
matugen: {
  pkgs,
  lib,
  config,
  ...
} @ inputs: let
  cfg = config.programs.matugen;
  osCfg = inputs.osConfig.programs.matugen or {};

  tomlFormat = pkgs.formats.toml {};
in {
  options.programs.matugen = {
    enable = lib.mkEnableOption "Matugen declarative theming";

    wallpaper = lib.mkOption {
      type = with lib.types; nullOr path;
      default = osCfg.wallpaper or null;

      example = "../wallpaper/astolfo.png";
      description = "Path to `wallpaper` that matugen will generate the colorschemes from";
    };

    templates = lib.mkOption {
      type = with lib;
        types.attrsOf (types.submodule {
          options = {
            input_path = mkOption {
              type = types.path;

              example = "./style.css";
              description = "Path to the template";
            };
            output_path = mkOption {
              type = types.str;

              example = ".config/style.css";
              description = "Path relative to your homedirectory where the generated file will be written to";
            };
          };
        });
      default = osCfg.templates or {};

      description = ''
        Templates that have `@{placeholders}` which will be replaced by the respective colors.
        See <https://github.com/InioX/matugen/wiki/Configuration#example-of-all-the-color-keywords> for a list of colors.
      '';
    };

    settings = lib.mkOption {
      inherit (tomlFormat) type;
      default = osCfg.settings or {};

      example = ''
        config.reload_apps_list = {
          gtk_theme = true;
          kitty = true;
        }
      '';
      description = ''
        Matugen configuration file in nix syntax.

        Written to {file}`$XDG_CONFIG_HOME/matugen/config.toml`
        A manual for configuring matugen can be found at <https://github.com/InioX/matugen/wiki/Configuration>.
      '';
    };
  };

  config = let
    package = matugen.packages.${pkgs.system}.default;

    mergedCfg =
      cfg.settings
      // (
        if cfg.templates != {}
        then {inherit (cfg) templates;}
        else {}
      );

    configFile = tomlFormat.generate "config.toml" mergedCfg;
  in
    lib.mkIf cfg.enable {
      home.packages = [package];

      home.activation.matugenCopyWallpapers = lib.hm.dag.entryAfter ["writeBoundary"] ''
        ${package}/bin/matugen image ${cfg.wallpaper} --config ${configFile}
      '';

      xdg.configFile."matugen/config.toml".source =
        lib.mkIf (mergedCfg != {}) configFile;
    };
}
