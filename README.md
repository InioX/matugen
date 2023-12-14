<div align="center">
     <img src="https://user-images.githubusercontent.com/81521595/226138807-db504bdf-4eb5-4fe9-9ee5-a1a1395d70dc.png" width=140>
      <h1>Matugen</h1>
 </div>
    
<div align="center">
     <img src="https://user-images.githubusercontent.com/81521595/236634805-15e68f9b-44a5-4efc-b275-0eb1f6a28bd9.gif" width="330" height="190"/>
     <br>
     <img alt="Crates.io" src="https://img.shields.io/crates/l/matugen?color=white&logo=license&style=for-the-badge">
     <img alt="Crates.io" src="https://img.shields.io/crates/v/matugen?color=white&logo=rust&style=for-the-badge">
     <br> 
     <a href="#-------------------------description">Description</a>
    ·
    <a href="#-------------------------installation">Installation</a>
    ·
    <a href="https://github.com/InioX/matugen/wiki">Wiki</a>
</div>

<div align="center">
  <sub>A cross-platform material you color generation tool
</div>
   
<h2 class="description">
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/da0dfc26-e8c0-46c1-ad13-bfaac394109b"
           height="25"
           width="25">
     </sub>
     Description
</h2>

Matugen is a cross-platform tool that generates a colorscheme either from an image or a color, and replaces keywords inside provided templates. It can also set the wallpaper if one was provided, and allows you to define your own keywords.


#### About Material Design 3
[Material Design 3](https://m3.material.io/) offers a new color system that allows for more flexible and dynamic use of color. The new system includes a wider range of colors, as well as a range of tints and shades that can be used to create subtle variations in color.

#### Other projects
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
          <img  src="https://github.com/InioX/matugen/assets/81521595/1ff9c777-94d9-4e24-85ec-725a6aa10612"
           height="25"
           width="25">
     </sub>
     Roadmap
</h2>

- [ ] Add GTK4 UI
- [x] Add a light/dark/amoled option for each template
- [x] Support more color formats for generating colorscheme
    - [x] Rgba
    - [x] Rgb
    - [x] Hsl
- [x] Suport changing the wallpaper on different platforms
     - [x] MacOS
     - [x] Windows
- [x] Support changing the wallpaper on X11
     - [x] Feh
     - [x] Nitrogen

> [!NOTE]
> Want a feature that is not listed above? Simply [open an issue](https://github.com/InioX/Matugen/issues).

<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/223f698f-9e72-430b-9a75-c9892fcea94e"
           height="25"
           width="25">
     </sub>
     Installation
</h2>

#### Cargo

<details><summary>Click to expand</summary>

```shell
cargo install matugen
```

</p>
</details>

#### NixOS

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

Option details can be found by reading the [module](./module.nix). A
[search.nixos.org](https://search.nixos.org/options)-like option viewer is
planned.

</p>
</details>

#### NetBSD

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
          <img  src="https://github.com/InioX/matugen/assets/81521595/bafdef83-4122-4bfd-9a30-98a5e0d7e488"
           height="25"
           width="25">
     </sub>
     Acknowledgements
</h2>

- [material-color-utilities-rs](https://github.com/alphaqu/material-color-utilities-rs)
- [wallpaper.rs](https://github.com/reujab/wallpaper.rs)
