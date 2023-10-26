# this arg is the matugen flake input
matugen: {
  pkgs,
  lib,
  config,
  ...
} @ args: let
  cfg = config.programs.matugen;
  osCfg = args.osConfig.programs.matugen or {};

  configFormat = pkgs.formats.toml {};

  capitalize = str: let
    inherit (builtins) substring stringLength;
    firstChar = substring 0 1 str;
    restOfString = substring 1 (stringLength str) str;
  in
    lib.concatStrings [(lib.toUpper firstChar) restOfString];

  # don't use ~, use $HOME
  sanitizedTemplates =
    builtins.mapAttrs (_: v: {
      mode = capitalize cfg.variant;
      input_path = builtins.toString v.input_path;
      output_path = builtins.replaceStrings ["$HOME"] ["~"] v.output_path;
    })
    cfg.templates;

  matugenConfig = configFormat.generate "matugen-config.toml" {
    config = {};
    templates = sanitizedTemplates;
  };

  # get matugen package
  pkg = matugen.packages.${pkgs.system}.default;

  themePackage = pkgs.runCommandLocal "matugen-themes-${cfg.variant}" {} ''
    mkdir -p $out
    cd $out
    export HOME=$(pwd)

    ${pkg}/bin/matugen \
      image ${cfg.wallpaper} \
      ${
      if cfg.templates != {}
      then "--config ${matugenConfig}"
      else ""
    } \
      --mode ${cfg.variant} \
      --palette ${cfg.palette} \
      --json ${cfg.jsonFormat} \
      --quiet \
      > $out/theme.json
  '';
  colors = builtins.fromJSON (builtins.readFile "${themePackage}/theme.json");
in {
  options.programs.matugen = {
    enable = lib.mkEnableOption "Matugen declarative theming";

    wallpaper = lib.mkOption {
      description = "Path to `wallpaper` that matugen will generate the colorschemes from";
      type = lib.types.path;
      default = osCfg.wallpaper or "${pkgs.nixos-artwork.wallpapers.simple-blue}/share/backgrounds/nixos/nix-wallpaper-simple-blue.png";
      defaultText = lib.literalExample ''
        "${pkgs.nixos-artwork.wallpapers.simple-blue}/share/backgrounds/nixos/nix-wallpaper-simple-blue.png"
      '';
    };

    templates = lib.mkOption {
      type = with lib.types;
        attrsOf (submodule {
          options = {
            input_path = lib.mkOption {
              type = path;
              description = "Path to the template";
              example = "./style.css";
            };
            output_path = lib.mkOption {
              type = str;
              description = "Path where the generated file will be written to";
              example = "~/.config/sytle.css";
            };
          };
        });
      default = osCfg.templates or {};
      description = ''
        Templates that have `@{placeholders}` which will be replaced by the respective colors.
        See <https://github.com/InioX/matugen/wiki/Configuration#example-of-all-the-color-keywords> for a list of colors.
      '';
    };

    palette = lib.mkOption {
      description = "Palette used when generating the colorschemes.";
      type = lib.types.enum ["default" "triadic" "adjacent"];
      default = osCfg.palette or "default";
      example = "triadic";
    };

    jsonFormat = lib.mkOption {
      description = "Color format of the colorschemes.";
      type = lib.types.enum ["rgb" "rgba" "hsl" "hsla" "hex" "strip"];
      default = osCfg.jsonFormat or "strip";
      example = "rgba";
    };

    variant = lib.mkOption {
      description = "Colorscheme variant.";
      type = lib.types.enum ["light" "dark" "amoled"];
      default = osCfg.variant or "dark";
      example = "light";
    };

    theme.files = lib.mkOption {
      type = lib.types.package;
      readOnly = true;
      default =
        if builtins.hasAttr "templates" osCfg
        then
          if cfg.templates != osCfg.templates
          then themePackage
          else osCfg.theme.files
        else themePackage;
      description = "Generated theme files. Including only the variant chosen.";
    };

    theme.colors = lib.mkOption {
      inherit (pkgs.formats.json {}) type;
      readOnly = true;
      default =
        if builtins.hasAttr "templates" osCfg
        then
          if cfg.templates != osCfg.templates
          then colors
          else osCfg.theme.colors
        else colors;
      description = "Generated theme colors. Includes all variants.";
    };
  };
}
