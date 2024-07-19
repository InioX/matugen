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

  matugenConfig = tomlFormat.generate "matugen-config.toml" {
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
      --type ${cfg.type} \
      --json ${cfg.jsonFormat} \
      --quiet \
      > $out/theme.json
  '';
  colors = builtins.fromJSON (builtins.readFile "${themePackage}/theme.json");
in {
  options.programs.matugen = {
    enable = lib.mkEnableOption "Matugen declarative theming";

    wallpaper = lib.mkOption {
      type = lib.types.path;
      default = osCfg.wallpaper or "${pkgs.nixos-artwork.wallpapers.simple-blue}/share/backgrounds/nixos/nix-wallpaper-simple-blue.png";

      description = "Path to `wallpaper` that matugen will generate the colorschemes from";
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

              example = "./style.css";
              description = "Path to the template";
            };
            output_path = lib.mkOption {
              type = str;

              example = "~/.config/sytle.css";
              description = "Path where the generated file will be written to";
            };
          };
        });
      default = osCfg.templates or {};

      description = ''
        Templates that have `@{placeholders}` which will be replaced by the respective colors.
        See <https://github.com/InioX/matugen/wiki/Configuration#example-of-all-the-color-keywords> for a list of colors.
      '';
    };

    type = lib.mkOption {
      type = lib.types.enum ["scheme-content" "scheme-expressive" "scheme-fidelity" "scheme-fruit-salad" "scheme-monochrome" "scheme-neutral" "scheme-rainbow" "scheme-tonal-spot"];
      default = osCfg.palette or "scheme-tonal-spot";
      
      example = "triadic";
      description = "Palette used when generating the colorschemes.";
    };

    jsonFormat = lib.mkOption {
      type = lib.types.enum ["rgb" "rgba" "hsl" "hsla" "hex" "strip"];
      default = osCfg.jsonFormat or "strip";
      
      example = "rgba";
      description = "Color format of the colorschemes.";
    };

    variant = lib.mkOption {
      type = lib.types.enum ["light" "dark" "amoled"];
      default = osCfg.variant or "dark";
      
      example = "light";
      description = "Colorscheme variant.";
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
