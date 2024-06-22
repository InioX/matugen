<div align="center">
     <img src="https://github.com/InioX/matugen/assets/81521595/66cfec75-702c-4b55-83fc-c474de171057" width=55% height=55%>
     <br><br>
     <img src="https://github.com/InioX/matugen/assets/81521595/ec3a165d-442d-4494-9aec-24254d11ae61" width=50% height=50%>
     <br><br>
     <img alt="license" src="https://custom-icon-badges.demolab.com/crates/l/matugen?color=3D3838&logo=law&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <img alt="version" src="https://custom-icon-badges.demolab.com/crates/v/matugen?color=3D3838&logo=package&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <br>
     <img alt="downloads" src="https://custom-icon-badges.demolab.com/crates/d/matugen?color=3D3838&logo=download&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <img alt="stars" src="https://custom-icon-badges.demolab.com/github/stars/InioX/matugen?color=3D3838&logo=star&style=for-the-badge&logoColor=370D10&labelColor=FEB3B3">
     <br> 
    <a href="#-------------------------installation">Installation</a>
    ·
    <a href="https://github.com/InioX/matugen/wiki">Wiki</a>
    ·
    <a href="#-------------------------themes">Themes</a>
</div>

<div align="center">
  <sub>A cross-platform material you color generation tool
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
- **Generate/Export Material You Color Palette:**
     - Generate a Material You color palette either from an image or a color
     - Output the generated palette as JSON to stdout, or use keywords inside templates that get exported as files
- **Keyword Filters:**
     - Use filters to change values of the keywords, like changing lightness for colors and manipulating strings with `replace`, `to_upper`, `to_lower` and `set_lightness`
- **Custom Keywords/Colors:**
     - Define your own custom keywords or colors you would like to be harmonized inside the config file, that you can then use in templates
- **Palette Customization:**
     - Customize the contrast and scheme type for the palette
- **Restart Apps/Change Wallpaper:**
     - Restart supported apps and set the wallpaper on Windows, MacOS, Linux and NetBSD

<br>
<div align="center">
<table>
  <tr>
     <th>
     <p>If you would like to learn more about the features and configuration, read the wiki <a href="https://github.com/InioX/matugen/wiki">here.</a></p>
     </th>
  </tr>
</table>
</div>

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

Using your favourite AUR helper:

```shell
yay -S matugen-bin
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
    ref = "refs/tags/matugen-v0.10.0"
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
