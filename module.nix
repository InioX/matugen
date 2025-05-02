# this arg is the matugen flake input
matugen: {
  pkgs,
  lib,
  config,
  ...
} @ args: let
  cfg = config.programs.matugen;
  osCfg = args.osConfig.programs.matugen or {};

  hexColorRegex = ''#([0-9a-fA-F]{3}){1,2}'';
  hexStrippedColorRegex = ''([0-9a-fA-F]{3}){1,2}'';
  rgbColorRegex = ''rgb\([0-9]{1,3}, ?[0-9]{1,3}, ?[0-9]{1,3}\)'';
  rgbaColorRegex = ''rgba\([0-9]{1,3}, ?[0-9]{1,3}, ?[0-9]{1,3}, ?[0-9]{1,3}\)'';
  hslColorRegex = ''hsl\([0-9]{1,3}(\.[0-9]*)?, ?[0-9]{1,3}(\.[0-9]*)?%, ?[0-9]{1,3}(\.[0-9]*)?%\)'';
  hslaColorRegex = ''hsla\([0-9]{1,3}(\.[0-9]*)?, ?[0-9]{1,3}(\.[0-9]*)?%, ?[0-9]{1,3}(\.[0-9]*)?%, ?[0,1](\.[0-9]*)?\)'';
  
  hexColor = lib.types.strMatching hexColorRegex;
  hexStrippedColor = lib.types.strMatching hexStrippedColorRegex;
  rgbColor = lib.types.strMatching rgbColorRegex;
  rgbaColor = lib.types.strMatching rgbaColorRegex;
  hslColor = lib.types.strMatching hslColorRegex;
  hslaColor = lib.types.strMatching hslaColorRegex;

  sourceColorType = lib.types.oneOf [hexColor rgbColor hslColor];
  customColorType = hexColor; # Only hexColor is currently supported for custom_colors.

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

  # takes in a source color string and returns the subcommand needed to generate
  # a color scheme using that color type.
  sourceColorTypeMatcher = color: (lib.lists.findSingle (p: null != builtins.match p.regex color) {} {} [
    { regex = hexColorRegex; code = "hex"; }
    { regex = rgbColorRegex; code = "rgb"; }
    { regex = hslColorRegex; code = "hsl"; }
  ]).code;

  command = if (builtins.isNull cfg.source_color) then
    "image ${cfg.wallpaper}" else
    "color ${sourceColorTypeMatcher cfg.source_color} \"${cfg.source_color}\"";

  themePackage = builtins.trace command (pkgs.runCommandLocal "matugen-themes-${cfg.variant}" {} ''
    mkdir -p $out
    cd $out
    export HOME=$(pwd)

    ${cfg.package}/bin/matugen \
      ${command} \
      --config ${matugenConfig} \
      --mode ${cfg.variant} \
      --type ${cfg.type} \
      --json ${cfg.jsonFormat} \
      --contrast ${lib.strings.floatToString cfg.contrast} \
      --lightness-dark ${lib.strings.floatToString cfg.lightness_dark} \
      --lightness-light ${lib.strings.floatToString cfg.lightness_light} \
      --quiet \
      > $out/theme.json
  '');
  colors = (builtins.fromJSON (builtins.readFile "${themePackage}/theme.json")).colors;
in {
  options.programs.matugen = {
    enable = lib.mkEnableOption "Matugen declarative theming";

    package =
      lib.mkPackageOption pkgs "matugen" {}
      // {
        default = pkg;
      };

    source_color = lib.mkOption {
      description = "Hex color to generate the colorschemes from. If this and wallpaper are defined, will use this.";
      type = lib.types.nullOr sourceColorType;
      default = osCfg.source_color or null;
      example = "#ff1243";
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
              type = customColorType;
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
      type = lib.types.enum ["scheme-content" "scheme-expressive" "scheme-fidelity" "scheme-fruit-salad" "scheme-monochrome" "scheme-neutral" "scheme-rainbow" "scheme-tonal-spot" "scheme-vibrant"];
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

    contrast = lib.mkOption {
      description = "Value from -1 to 1. -1 represents minimum contrast, 0 represents standard (i.e. the design as spec'd), and 1 represents maximum contrast.";
      type = lib.types.numbers.between (-1) 1;
      default = 0;
      example = "0.2";   
    };
    
    lightness_dark = lib.mkOption {
      description = "Value from -∞ to 1. -∞ represents minimum lightness, 0 represents standard (i.e. the design as spec'd), and 1 represents maximum lightness. For dark schemes, if the considered lightnesses are between 0 and 1 then this applies an affine transformation to the lightness by keeping the value for 1 at 1 and setting the value for 0 to the lightness argument and then clamping the result";
      type = lib.types.addCheck lib.types.number (lightness_dark: lightness_dark <= 1);
      default = 0;
      example = "0.2";
      
      check = lightness_light: lightness_light >= -1;
    };

    lightness_light = lib.mkOption {
      description = "Value from -1 to +∞. -1 represents minimum lightness, 0 represents standard (i.e. the design as spec'd), and +∞ represents maximum lightness. For light schemes, if the considered lightnesses are between 0 and 1 then this applies an affine transformation to the lightness by keeping the value for 0 at 0 and setting the value for 1 to (1 + the lightness argument) and then clamping the result";
      type = lib.types.addCheck lib.types.number (lightness_light: lightness_light >= -1);
      default = 0;
      example = "0.2";
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
