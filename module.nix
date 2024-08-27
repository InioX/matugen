# this arg is the matugen flake input
matugen: {
  pkgs,
  lib,
  config,
  ...
} @ args: let
  cfg = config.programs.matugen;
  osCfg = args.osConfig.programs.matugen or {};

  hexColor = lib.types.strMatching "#([0-9a-fA-F]{3}){1,2}";
  hexStrippedColor = lib.types.strMatching "([0-9a-fA-F]{3}){1,2}";
  rgbColor = lib.types.strMatching "rgb\(\d{1,3}, ?\d{1,3}, ?\d{1,3}\)";
  rgbaColor = lib.types.strMatching "rgba\(\d{1,3}, ?\d{1,3}, ?\d{1,3}, ?\d{1,3}\)";
  hslColor = lib.types.strMatching "hsl\(\d{1,3}, ?\d{1,3}%, ?\d{1,3}%\)";
  hslaColor = lib.types.strMatching "hsla\(\d{1,3}, ?\d{1,3}%, ?\d{1,3}%, ?[0,1](\.\d*)\)";

  # Only hexColor is currently supported for custom_colors.
  colorType = hexColor;
  # colorType = lib.types.oneOf [hexColor hexStrippedColor rgbColor rgbaColor hslColor hslaColor];

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
    config = {
      custom_colors = cfg.custom_colors;
    } // cfg.config;
    templates = sanitizedTemplates;
  };

  # get matugen package
  pkg = matugen.packages.${pkgs.system}.default;

  themePackage = pkgs.runCommandLocal "matugen-themes-${cfg.variant}" {} ''
    mkdir -p $out
    cd $out
    export HOME=$(pwd)

    ${cfg.package}/bin/matugen \
      image ${cfg.wallpaper} \
      --config ${matugenConfig} \
      --mode ${cfg.variant} \
      --type ${cfg.type} \
      --json ${cfg.jsonFormat} \
      --quiet \
      > $out/theme.json
  '';
  colors = (builtins.fromJSON (builtins.readFile "${themePackage}/theme.json")).colors;
in {
  options.programs.matugen = {
    enable = lib.mkEnableOption "Matugen declarative theming";

    package =
      lib.mkPackageOption pkgs "matugen" {}
      // {
        default = pkg;
      };

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

    custom_colors = lib.mkOption {
      description = "Other colors that should be included in the colorsheme.";
      type = with lib.types;
        attrsOf (submodule {
          options = {
            color = lib.mkOption {
              description = "Color value for this custom color.";
              type = colorType;
              example = "#d03e3e";
            };
            blend = lib.mkOption {
              description = "Whether to pick a color close to the given value, or to pass the value through to the final colorscheme unchanged.";
              type = bool;
              default = true;
            };
          };
        });
      default = osCfg.custom_colors or {};
      example = ''
        {
          light-red.color = "#d03e3e";
          light-orange.color = "#d7691d";
          light-yellow.color = "#ad8200";

          red = {
            color = "#ff0000";
            blend = false;
          };
        }
      '';
    };

    type = lib.mkOption {
      description = "Palette used when generating the colorschemes.";
      type = lib.types.enum ["scheme-content" "scheme-expressive" "scheme-fidelity" "scheme-fruit-salad" "scheme-monochrome" "scheme-neutral" "scheme-rainbow" "scheme-tonal-spot"];
      default = osCfg.palette or "scheme-tonal-spot";
      example = "scheme-content";
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

    config = lib.mkOption {
      description = "Add things to the config not covered by other options.";
      type = lib.types.attrs;
      default = osCfg.config or {};
      example = ''
        {
          custom_keywords.font1 = "Google Sans";
        }
      '';
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
