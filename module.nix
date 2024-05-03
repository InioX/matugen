matugen: {
  config,
  pkgs,
  lib,
  ...
} @ inputs:
with lib; let
  cfg = config.programs.matugen;
  osCfg = inputs.osConfig.programs.matugen or {};

  tomlFormat = pkgs.formats.toml {};

  templateModule = types.submodule {
    options = {
      input_path = mkOption {
        type = types.path;

        example = "./gtk.css";
        description = "Path to a matugen template file.";
      };

      output_path = mkOption {
        type = types.str;

        example = "$XDG_CACHE_HOME/wal/colors.json";
        description = "Destination path where the processed template files should be stored.";
      };
    };
  };
in {
  options.programs.matugen = {
    enable =
      mkEnableOption
      {
        default = osCfg.enable or {};
        description = "Whether to enable Matugen.";
      };

    settings = mkOption {
      type = tomlFormat.type;
      default = osCfg.settings or {};

      example = ''
        config.reload_apps_list = {
          gtk_theme = true;
          kitty = true;
        };
      '';
      description = ''
        Matugen configuration file in nix syntax.

        Written to {file}`$XDG_CONFIG_HOME/matugen/config.toml`
        A manual for configuring matugen can be found at <https://github.com/InioX/matugen/wiki/Configuration>.
      '';
    };

    templates = mkOption {
      type = types.attrsOf templateModule;
      default = osCfg.templates or {};

      example = ''
        {
          input_path = ./gtk.css;
          output_path = "$XDG_CACHE_HOME/wal/colors.json";
        }
      '';
      description = ''
        Templates that matugen is supposed to complete.
        A guide to writing templates can be found here <https://github.com/InioX/matugen/wiki/Configuration>.
      '';
    };

    wallpaper = mkOption {
      type = with types; nullOr path;
      default = osCfg.wallpaper or null;

      example = "../wallpaper/astolfo.png";
      description = "Wallpaper to use when decleratively using matugen. If omited, The wallpaper can only be configured imperatively.";
    };
  };

  config = let
    package = matugen.packages.${pkgs.system}.default;

    mergedCfg =
      cfg.settings
      // (
        if cfg.templates != {}
        then {templates = cfg.templates;}
        else {}
      );

    resultsPackage =
      if (cfg.wallpaper != null)
      # TODO this doesn't work because of the $out prefix I'd need to write to.
      then [(pkgs.runCommand "results" {} "${package}/bin/matugen image ${cfg.wallpaper}")]
      else [];
  in
    mkIf cfg.enable {
      home.packages =
        [package]
        ++ resultsPackage;

      xdg.configFile."matugen/config.toml".source =
        mkIf (mergedCfg != {}) (tomlFormat.generate "config.toml" mergedCfg);
    };
}
