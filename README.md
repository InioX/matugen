<div align="center">
     <img src="https://github.com/InioX/matugen/assets/81521595/66cfec75-702c-4b55-83fc-c474de171057" width=55% height=55%>
     <br><br>
     <img src="https://github.com/InioX/matugen/assets/81521595/ec3a165d-442d-4494-9aec-24254d11ae61" width=50% height=50%>
     <br>
     A cross-platform material you and base16 color generation tool<br>
     <sub>(pronounced: mat-uh-gen)</sub>
     <br><br>
     <img alt="license" src="https://custom-icon-badges.demolab.com/crates/l/matugen?color=3D3838&logo=law&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <img alt="version" src="https://custom-icon-badges.demolab.com/crates/v/matugen?color=3D3838&logo=package&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <br>
     <img alt="downloads" src="https://custom-icon-badges.demolab.com/crates/d/matugen?color=3D3838&logo=download&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <img alt="stars" src="https://custom-icon-badges.demolab.com/github/stars/InioX/matugen?color=3D3838&logo=star&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <br> 
    <a href="#-------------------------installation">Installation</a>
    ·
    <a href="https://iniox.github.io/#matugen">Wiki</a>
    ·
    <a href="#--------------------themes">Themes</a>
</div>

<div align="center">
  <img src="https://github.com/InioX/matugen/assets/81521595/9008d8d9-0157-4b38-9500-597986a2cb9f" height=35% width=35%>
</div>

<div align="center">
     <a href="https://ko-fi.com/iniox">
          <img src="https://ziadoua.github.io/m3-Markdown-Badges/badges/Ko-fi/ko-fi2.svg">
     </a>
     <a href="https://discord.gg/JA3C2U9EcC">
          <img src="https://github.com/user-attachments/assets/84f4e3a2-73ac-4112-8633-c02164c2c02c">
     </a>
     <br>
     <sub>Donations through Ko-Fi are welcome!
</div>

<div align="center">
  <img src="https://github.com/InioX/matugen/assets/81521595/9008d8d9-0157-4b38-9500-597986a2cb9f">
</div>

<h2 class="description">
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/da0dfc26-e8c0-46c1-ad13-bfaac394109b"
           height="25"
           width="25">
     </sub>
     Description
</h2>

### Features

> [!TIP]
> If you would like to learn more about the features and configuration, read the wiki <a href="https://iniox.github.io/#matugen">here.</a>

- **Templating engine built with Chumsky (designed for colors)** - [Read More](https://iniox.github.io/#matugen/templates)
  - Custom engine focused on making color manipulation simple and efficient
  - Import a color once in any format and automatically access all other formats (`hex`, `rgb`, `rgba`, `hsl`, etc.)
  - Colors are parsed as real color objects, not strings, making filters faster and more reliable
  - Supports piping, nested expressions, conditionals, loops, filters, includes, arithmetic operations, escaping output.
  - Can be used as a standalone templating engine. You can import custom json files or define and override them in the CLI.

- **Generate / Export Material You color palettes**
  - Generate a full Material You palette from either an image or a single color
  - Export the generated palette as JSON or reference palette keywords directly within templates
  - Easily integrate palette values into config files, themes, or style templates

- **Keyword Filters** - [Read More](https://iniox.github.io/#matugen/filters)
  - Modify any keyword using filters such as `replace`, `to_upper`, `to_lower`, and `set_lightness`
  - Includes built-in color filters for adjusting hue, saturation, lightness, opacity, and more
  - Filters can be chained together for powerful inline transformations

- **Custom Keywords / Colors**
  - Import any JSON file (through CLI or config) and use its contents directly inside templates
  - Imported colors receive full multi-format support, just like built-in palette colors
  - Useful for adding custom theme data, config variables, or full color schemes

- **Palette Customization**
  - Adjust contrast, lightness and choose the scheme type (light, dark, or custom variants)
  - Fine-tune the generated palette to match your preference or application theme

### Other projects
- [Mitsugen](https://github.com/DimitrisMilonopoulos/mitsugen) - For gnome-shell, based on the [old](https://github.com/InioX/matugen/tree/python) version of Matugen
- [pywal](https://github.com/dylanaraps/pywal) - More color generation backends, default theme files. 
- [wpgtk](https://github.com/deviantfero/wpgtk) - Like pywal, but with a gui and more features.
  
<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/3c01525a-c8b1-499e-9f28-a17e81edfb5b"
           height="25"
           width="25">
     </sub>
     Supported platforms
</h2>

- Windows
- Linux
- MacOS
- NetBSD

<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/223f698f-9e72-430b-9a75-c9892fcea94e"
           height="25"
           width="25">
     </sub>
     Installation
</h2>

<h4>
     <sub>
          <img  src="https://cdn.simpleicons.org/rust/white"
           height="20"
           width="20">
     </sub>
     Cargo
     <a href="https://crates.io/crates/matugen"><img alt="Cargo Version" src="https://img.shields.io/crates/v/matugen?color=brightgreen&label=" align="right"></a>
</h4>

<details><summary>Click to expand</summary>

```shell
cargo install matugen
```

</p>
</details>

<h4>
     <sub>
          <img  src="https://cdn.simpleicons.org/archlinux/white"
           height="20"
           width="20">
     </sub>
     Arch
     <a href="https://aur.archlinux.org/packages/matugen-bin"><img alt="AUR Version" src="https://img.shields.io/aur/version/matugen-bin?color=brightgreen&label=" align="right"></a>
</h4>

<details><summary>Click to expand</summary>

```shell
sudo pacman -S matugen
```

</p>
</details>

<h4>
     <sub>
          <img  src="https://cdn.simpleicons.org/nixos/white"
           height="20"
           width="20">
     </sub>
     NixOS
     <a href="https://repology.org/project/matugen/versions">
  <img src="https://repology.org/badge/version-for-repo/nix_stable_24_05/matugen.svg?header=" alt="nixpkgs" align="right">
     </a><a href="j"><img alt="NixOS Version" src="https://img.shields.io/badge/git-brightgreen" align="right"></a>
</h4>

<details><summary>Click to expand</summary>
<p>

Add matugen to your flake inputs:
```nix
inputs = {
  matugen = {
    url = "github:/InioX/Matugen";
    # If you need a specific version:
    ref = "refs/tags/matugen-v0.10.0";
  };
  # ...
};
```

Then you can add it to your packages:
```nix
let
  system = "x86_64-linux";
in {
  environment.systemPackages = with pkgs; [    
    # ...
    inputs.matugen.packages.${system}.default
  ];
}
```

This flake also provides a NixOS/Home Manager module, which can be imported by
adding this in your configuration:
```nix
{pkgs, inputs, ...}: {
  imports = [
    inputs.matugen.nixosModules.default
  ];

  # ...
}
```

The module does NOT automatically symlink the files. For an example of using this module with Home Manager, see https://github.com/InioX/matugen/issues/28

Option details can be found by reading the [module](./module.nix). A
[search.nixos.org](https://search.nixos.org/options)-like option viewer is
planned.

</p>
</details>

<h4>
     <sub>
          <img  src="https://cdn.simpleicons.org/netbsd/white"
           height="20"
           width="20">
     </sub>
     NetBSD
     <a href="https://repology.org/project/matugen/versions">
  <img src="https://repology.org/badge/version-for-repo/pkgsrc_current/matugen.svg?header=" alt="pkgsrc current package" align="right"></a>
</h4>

<details><summary>Click to expand</summary>

```shell
pkgin install matugen
```
or, if you prefer to build it from source
```shell
cd /usr/pkgsrc/graphics/matugen
make install
```

</p>
</details>


<h2>
     <sub>
     <img src="https://github.com/InioX/matugen-themes/assets/81521595/5e0b21af-62da-44ad-9492-f25689b260d9"
           height="25"
           width="25">
     </sub>
     Themes
</h2>

#### Templates
- [InioX/matugen-themes](https://github.com/InioX/matugen-themes)

<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/bafdef83-4122-4bfd-9a30-98a5e0d7e488"
           height="25"
           width="25">
     </sub>
     Acknowledgements
</h2>

- [material-colors](https://github.com/Aiving/material-colors)
- [wallpaper.rs](https://github.com/reujab/wallpaper.rs) - Changing wallpaper for Windows
