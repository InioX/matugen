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
     <a href="#installation">Installation</a>
    ·
    <a href="#usage">Usage</a>
    ·
    <a href="#configuration">Configuration</a>
</div>

<div align="center">
  <sub>A cross-platform material you color generation tool
</div>
   
<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/da0dfc26-e8c0-46c1-ad13-bfaac394109b"
           height="25"
           width="25">
     </sub>
     Description
</h2>

Matugen is a cross-platform tool that generates a colorscheme either from an image or a color, and exports it to a file from a template. It can also set the wallpaper if one was provided.


### About Material Design 3
[Material Design 3](https://m3.material.io/) offers a new color system that allows for more flexible and dynamic use of color. The new system includes a wider range of colors, as well as a range of tints and shades that can be used to create subtle variations in color.

### Other projects
- [Mitsugen](https://github.com/DimitrisMilonopoulos/mitsugen) - For gnome-shell, based on the [old](https://github.com/InioX/matugen/tree/python) and deprecated python version of Matugen

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
> **Warning**
> Matugen only supports setting the wallpaper and restarting apps on Linux and NetBSD for now.


<h2>
     <sub>
          <img  src="https://cdn.discordapp.com/attachments/1107367450909081662/1156867978340606002/outline_checklist_white_24dp.png?ex=651688c3&is=65153743&hm=64edeb20edebe3dcaf752638fed5d7d218de4033973aef1e728441b05ad9a486&"
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
- [ ] Suport changing the wallpaper on different platforms
     - [ ] MacOS
     - [ ] Windows
- [x] Support changing the wallpaper on X11
     - [x] Feh
     - [x] Nitrogen

> **Note**
> Want a feature that is not listed above? Simply [open an issue](https://github.com/InioX/Matugen/issues).

<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/223f698f-9e72-430b-9a75-c9892fcea94e"
           height="25"
           width="25">
     </sub>
     Installation
</h2>

### Cargo

```shell
cargo install matugen
```

### NixOS
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

### NetBSD
```shell
pkgin install matugen
```
or, if you prefer to build it from source
```shell
cd /usr/pkgsrc/graphics/matugen
make install
```

## Usage

### Modes
<img src="./assets/images/modes.gif" width=450>

<table>
<tr>
    <td>Light</td>
    <td>Dark</td>
    <td>Amoled</sup></td>
  </tr>
    <tr>
    <td><img src="https://media.discordapp.net/attachments/1134177615964545024/1140270597381832774/image.png"></td>
    <td><img src="https://media.discordapp.net/attachments/1134177615964545024/1140270155205713920/image.png"></td>
    <td><img src="https://media.discordapp.net/attachments/1134177615964545024/1140270375956119623/image.png"></td>
  </tr>
</table>

### Palettes
<img src="./assets/images/palette.gif" width=450>

<table>
<tr>
    <td>Default</td>
    <td>Triadic</td>
    <td>Adjacent</sup></td>
  </tr>
    <tr>
    <td><img src="https://cdn.discordapp.com/attachments/1134177615964545024/1140342805013725244/image.png"></td>
    <td><img src="https://cdn.discordapp.com/attachments/1134177615964545024/1140342896525049866/image.png"></td>
    <td><img src="https://cdn.discordapp.com/attachments/1134177615964545024/1140342950262472776/image.png"></td>
  </tr>
</table>

### Json flag
Allows for dumping the schemes similarly to `--show-colors`, but in a
machine-readable format. Can dump hex, rgba, hsl, etc.

<details><summary>Result</summary>
<p>
     
```json
{
  "colors": {
    "amoled": {
      "background": "#000000",
      "error": "#ffb4ab",
      "error_container": "#93000a",
      "inverse_on_surface": "#323032",
      "inverse_primary": "#72518b",
      "inverse_surface": "#e7e1e4",
      "on_background": "#e7e1e4",
      "on_error": "#690005",
      "on_error_container": "#ffb4ab",
      "on_primary": "#412259",
      "on_primary_container": "#f2daff",
      "on_secondary": "#362d3b",
      "on_secondary_container": "#ecdef1",
      "on_surface": "#e7e1e4",
      "on_surface_variant": "#ccc4cc",
      "on_tertiary": "#47282a",
      "on_tertiary_container": "#ffdadb",
      "outline": "#958f96",
      "outline_variant": "#4a454c",
      "primary": "#dfb8fa",
      "primary_container": "#593972",
      "scrim": "#000000",
      "secondary": "#cfc2d4",
      "secondary_container": "#4d4352",
      "shadow": "#000000",
      "source_color": "#bb96d6",
      "surface": "#000000",
      "surface_variant": "#131015",
      "tertiary": "#ebbbbd",
      "tertiary_container": "#603d40"
    },
    "dark": {
      "background": "#1d1b1d",
      "error": "#ffb4ab",
      "error_container": "#93000a",
      "inverse_on_surface": "#323032",
      "inverse_primary": "#72518b",
      "inverse_surface": "#e7e1e4",
      "on_background": "#e7e1e4",
      "on_error": "#690005",
      "on_error_container": "#ffb4ab",
      "on_primary": "#412259",
      "on_primary_container": "#f2daff",
      "on_secondary": "#362d3b",
      "on_secondary_container": "#ecdef1",
      "on_surface": "#e7e1e4",
      "on_surface_variant": "#ccc4cc",
      "on_tertiary": "#47282a",
      "on_tertiary_container": "#ffdadb",
      "outline": "#958f96",
      "outline_variant": "#4a454c",
      "primary": "#dfb8fa",
      "primary_container": "#593972",
      "scrim": "#000000",
      "secondary": "#cfc2d4",
      "secondary_container": "#4d4352",
      "shadow": "#000000",
      "source_color": "#bb96d6",
      "surface": "#1d1b1d",
      "surface_variant": "#4a454c",
      "tertiary": "#ebbbbd",
      "tertiary_container": "#603d40"
    },
    "light": {
      "background": "#fffbff",
      "error": "#ba1a1a",
      "error_container": "#ffdad6",
      "inverse_on_surface": "#f5eff2",
      "inverse_primary": "#dfb8fa",
      "inverse_surface": "#323032",
      "on_background": "#1d1b1d",
      "on_error": "#ffffff",
      "on_error_container": "#410002",
      "on_primary": "#ffffff",
      "on_primary_container": "#2a0a43",
      "on_secondary": "#ffffff",
      "on_secondary_container": "#201926",
      "on_surface": "#1d1b1d",
      "on_surface_variant": "#4a454c",
      "on_tertiary": "#ffffff",
      "on_tertiary_container": "#2e1316",
      "outline": "#7b757c",
      "outline_variant": "#ccc4cc",
      "primary": "#72518b",
      "primary_container": "#f2daff",
      "scrim": "#000000",
      "secondary": "#655b6a",
      "secondary_container": "#ecdef1",
      "shadow": "#000000",
      "source_color": "#bb96d6",
      "surface": "#fffbff",
      "surface_variant": "#e9e0e8",
      "tertiary": "#7a5557",
      "tertiary_container": "#ffdadb"
    }
  },
  "colors_android": {
    "amoled": {
      "accent_surface": "#fbecff",
      "color_accent_primary": "#f2daff",
      "color_accent_primary_variant": "#c29ddd",
      "color_accent_secondary": "#ecdef1",
      "color_accent_secondary_variant": "#b4a7b9",
      "color_accent_tertiary": "#ffdadb",
      "color_accent_tertiary_variant": "#cda0a3",
      "color_background": "#000000",
      "color_background_floating": "#000000",
      "color_surface": "#121013",
      "color_surface_highlight": "#1d1b1d",
      "color_surface_variant": "#272528",
      "off_state": "#323032",
      "scrim_android": "#cac5c8",
      "surface_header": "#1d1b1d",
      "text_color_primary": "#f5eff2",
      "text_color_primary_inverse": "#1d1b1d",
      "text_color_secondary": "#ccc4cc",
      "text_color_secondary_inverse": "#494649",
      "text_color_tertiary": "#958f96",
      "text_color_tertiary_inverse": "#7a7679",
      "text_primary_on_accent": "#1d1b1d",
      "text_secondary_on_accent": "#4a454c",
      "under_surface": "#000000",
      "volume_background": "#000000"
    },
    "dark": {
      "accent_surface": "#fbecff",
      "color_accent_primary": "#f2daff",
      "color_accent_primary_variant": "#c29ddd",
      "color_accent_secondary": "#ecdef1",
      "color_accent_secondary_variant": "#b4a7b9",
      "color_accent_tertiary": "#ffdadb",
      "color_accent_tertiary_variant": "#cda0a3",
      "color_background": "#1d1b1d",
      "color_background_floating": "#1d1b1d",
      "color_surface": "#323032",
      "color_surface_highlight": "#555154",
      "color_surface_variant": "#494649",
      "off_state": "#323032",
      "scrim_android": "#cac5c8",
      "surface_header": "#494649",
      "text_color_primary": "#f5eff2",
      "text_color_primary_inverse": "#1d1b1d",
      "text_color_secondary": "#ccc4cc",
      "text_color_secondary_inverse": "#494649",
      "text_color_tertiary": "#958f96",
      "text_color_tertiary_inverse": "#7a7679",
      "text_primary_on_accent": "#1d1b1d",
      "text_secondary_on_accent": "#4a454c",
      "under_surface": "#000000",
      "volume_background": "#3d3a3d"
    },
    "light": {
      "accent_surface": "#fbecff",
      "color_accent_primary": "#f2daff",
      "color_accent_primary_variant": "#72518b",
      "color_accent_secondary": "#ecdef1",
      "color_accent_secondary_variant": "#655b6a",
      "color_accent_tertiary": "#ffdadb",
      "color_accent_tertiary_variant": "#7a5557",
      "color_background": "#f5eff2",
      "color_background_floating": "#fef8fb",
      "color_surface": "#fef8fb",
      "color_surface_highlight": "#ffffff",
      "color_surface_variant": "#e7e1e4",
      "off_state": "#323032",
      "scrim_android": "#cac5c8",
      "surface_header": "#e7e1e4",
      "text_color_primary": "#1d1b1d",
      "text_color_primary_inverse": "#f5eff2",
      "text_color_secondary": "#4a454c",
      "text_color_secondary_inverse": "#cac5c8",
      "text_color_tertiary": "#7b757c",
      "text_color_tertiary_inverse": "#948f93",
      "text_primary_on_accent": "#1d1b1d",
      "text_secondary_on_accent": "#4a454c",
      "under_surface": "#000000",
      "volume_background": "#3d3a3d"
    }
  }
}
```
     
</p>
</details>

```sh
matugen --json <JSON> <other-arguments>
```

### Help
<img src="./assets/images/help.gif" width=450>

```sh
matugen -h
matugen --help
```

### Show colors
<img src="https://media.discordapp.net/attachments/1134177615964545024/1140373989294874764/image.png?width=837&height=684" width=300>

```sh
matugen --show-colors <other-arguments>
```

### Verbose mode
<img src="./assets/images/verbose.gif" width=450>

```sh
matugen -v <other-arguments>
```
     
### Generate from an image
<img src="./assets/images/image.gif" width=450>

```sh
# Dark mode
matugen image /path/to/wallpaper/ -m "dark"
# Light mode
matugen image /path/to/wallpaper/ -m "light"
# AMOLED/"pure dark" mode
matugen image /path/to/wallpaper/ -m "amoled"

```
Example:
```sh
matugen image ~/wall/snow.png -l
```
     
### Generate from a color
<img src="./assets/images/color.gif" width=450>

```sh
# Dark mode
matugen color hsl <hsl color> -m "dark"
# Light mode
matugen color hex <hex color> -m "light"
# AMOLED/"pure dark" mode
matugen color rgb <rgb color> -m "amoled"
```
Example:
```sh
matugen color hex "#ffbf9b"
matugen color rgb "rgb(63, 106, 171)" -m "light"
matugen color hsl "hsl(216.34, 45.75%, 45.88%)" -m "amoled"
```

### Creating templates
The basic syntax for using colors is `prefix + {color}` (The default prefix is `@`, so the usage would be `@{color}`).

#### Keywords
If you want a specific scheme, you can use `@{primary.light.hex}`. All available modes/schemes can be found <a href="#usage">here</a>.

```css
@define-color primary @{primary}; /* Result: #ffb783 */
@define-color primary @{primary.hex}; /* Result: #ffb783 */
@define-color primary @{primary.rgb}; /* Result: rgb(255, 183, 131) */
@define-color primary @{primary.rgba}; /* Result: rgba(255, 183, 131, 255) */
@define-color primary @{primary.strip}; /* Result: ffb783 */
@define-color primary @{primary.hsl}; /* Result: hsl(25, 100%, 76%) */
@define-color primary @{primary.hsla}; /* Result: hsla(242.2, 49.4%, 67.45%, 1) */

@define-color primary @{background.light.hex}; /* Result: #fffbff */
@define-color primary @{background.dark.hex}; /* Result: #1e1b19 */
@define-color primary @{background.amoled.hex}; /* Result: #000000 */
```

You can get the source color (color used for generating colorscheme) by using:
```css
@import url("@{source_color}"); /* Result: #ffb783*/
```

You can also get the image (if it was provided) by using:
```css
@import url("@{image}"); /* Result: /home/ini/Downloads/wallpaper.jpg */
```
> **Note**
> If no image was provided, Matugen will just skip over the image keyword

#### Example of all the color keywords:
```css
/*colors.css*/
@define-color primary @{primary};
@define-color on_primary @{on_primary};
@define-color primary_container @{primary_container};
@define-color on_primary_container @{on_primary_container};
@define-color secondary @{secondary};
@define-color on_secondary @{on_secondary};
@define-color secondary_container @{secondary_container};
@define-color on_secondary_container @{on_secondary_container};
@define-color tertiary @{tertiary};
@define-color on_tertiary @{on_tertiary};
@define-color tertiary_container @{tertiary_container};
@define-color on_tertiary_container @{on_tertiary_container};
@define-color error @{error};
@define-color on_error @{on_error};
@define-color error_container @{error_container};
@define-color on_error_container @{on_error_container};
@define-color background @{background};
@define-color on_background @{on_background};
@define-color surface @{surface};
@define-color on_surface @{on_surface};
@define-color surface_variant @{surface_variant};
@define-color on_surface_variant @{on_surface_variant};
@define-color outline @{outline};
@define-color shadow @{shadow};
@define-color scrim @{scrim};
@define-color inverse_surface @{inverse_surface};
@define-color inverse_on_surface @{inverse_on_surface};
@define-color inverse_primary @{inverse_primary};

@define-color source_color @{source_color};
@define-color color_accent_primary @{color_accent_primary};
@define-color color_accent_primary_variant @{color_accent_primary_variant};
@define-color color_accent_secondary @{color_accent_secondary};
@define-color color_accent_secondary_variant @{color_accent_secondary_variant};
@define-color color_accent_tertiary @{color_accent_tertiary};
@define-color color_accent_tertiary_variant @{color_accent_tertiary_variant};
@define-color text_color_primary @{text_color_primary};
@define-color text_color_secondary @{text_color_secondary};
@define-color text_color_tertiary @{text_color_tertiary};
@define-color text_color_primary_inverse @{text_color_primary_inverse};
@define-color text_color_secondary_inverse @{text_color_secondary_inverse};
@define-color text_color_tertiary_inverse @{text_color_tertiary_inverse};
@define-color color_background @{color_background};
@define-color color_background_floating @{color_background_floating};
@define-color color_surface @{color_surface};
@define-color color_surface_variant @{color_surface_variant};
@define-color color_surface_highlight @{color_surface_highlight};
@define-color surface_header @{surface_header};
@define-color under_surface @{under_surface};
@define-color off_state @{off_state};
@define-color accent_surface @{accent_surface};
@define-color text_primary_on_accent @{text_primary_on_accent};
@define-color text_secondary_on_accent @{text_secondary_on_accent};
@define-color volume_background @{volume_background};
@define-color scrim_android @{scrim_android};
```

## Configuration
Here is a list of different locations for the configuration file:
- Windows: `C:\Users\user\AppData\Roaming\InioX\matugen\config\config.toml`
- Linux: `/home/user/.config/matugen/config.toml`
- MacOS: `/Users/user/Library/Application Support/com.InioX.matugen/config.toml`

> **Note**
> You can also use a custom configuration path by using the `-c` argument

### Configuration items
| Name                 | Type          | Default   | Description                                                                                     |
|----------------------|---------------|-----------|-------------------------------------------------------------------------------------------------|
| reload_apps          | bool          | false     | Whether to reload apps.                                                                         |
| set_wallpaper        | bool          | false     | Whether to set the wallpaper (if `true`, requires `wallpaper_tool` to be set).                  |
| wallpaper_tool       | String        | None      | The wallpaper tool to use (`Swwww`, `Swaybg`, `Feh`, `Nitrogen`).                               |
| prefix               | String        | "@"       | The prefix to use (for example: `@{primary}`)                                                   |
| ~~reload_gtk_theme~~ | ~~bool~~      | ~~false~~ | ~~Whether to reload the gtk theme.~~ **REMOVED, USE `gtk_theme` in `config.reload_apps_list`.** |
| run_after            | Vec<String>   | []        | The commands to run after the templates have been generated.                                    |
| swww_options         | <Vec<String>> | []        | The options to use for [Swwww](https://github.com/Horus645/swww)                                |
| feh_options          | <Vec<String>> | []        | The options to use for [Feh](https://github.com/derf/feh)                                       |

### Apps
| Name      | Type | Default | Description                      |
|-----------|------|---------|----------------------------------|
| kitty     | bool | true    | Whether to reload kitty.         |
| waybar    | bool | true    | Whether to reload waybar.        |
| dunst     | bool | true    | Whether to reload dunst.         |
| gtk_theme | bool | true    | Whether to reload the GTK theme. |

### Example
```toml
# config_directory/config.toml
[config]
reload_apps = true
set_wallpaper = true
wallpaper_tool = 'Swww'
prefix = '@'
swww_options = [
    "--transition-type",
    "center",
]
run_after = [
    [ "echo", "'hello'" ],
    [ "echo", "'hello again'" ],
]

[config.reload_apps_list]
waybar = true
kitty = true
gtk_theme = true
dunst = true
```

### Adding templates
| Name            | Type                  | Default                   | Description                             |
|-----------------|-----------------------|---------------------------|-----------------------------------------|
| mode            | Option\<Modes\>       | Mode provided in args     | Which scheme to use for the template.   |
| input_path      | PathBuf               | None                      | Path to the template file.              |
| output_path     | PathBuf               | None                      | Path to export the template to.         |

### Modes
>**Note** The `mode` key will override the mode specified in the arguments, only for that template.

For all available modes, look at the <a href="#usage">usage</a>.

### Example
```toml
# config_directory/config.toml

[templates.test] # First way of adding template
input_path = '~/.config/example/template.css'
output_path = '~/.config/example'
mode = "Light" # First letter MUST be upper-case

[templates] # Another way
test2 = { input_path = '~/.config/example/template2.css', output_path = '~/.config/example2' }
```

<h2>
     <sub>
          <img  src="https://github.com/InioX/matugen/assets/81521595/bafdef83-4122-4bfd-9a30-98a5e0d7e488"
           height="25"
           width="25">
     </sub>
     Acknowledgements
</h2>

- [material-color-utilities-rs](https://github.com/alphaqu/material-color-utilities-rs)
