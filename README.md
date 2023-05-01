<div align="center">
     <img src="https://user-images.githubusercontent.com/81521595/226138807-db504bdf-4eb5-4fe9-9ee5-a1a1395d70dc.png" width=140>
      <h1>Matugen</h1>
 </div>
    
<div align="center">
    <a href="#installation">Installation</a>
    ·
    <a href="#usage">Usage</a>
    ·
    <a href="https://github.com/InioX/matugen/wiki">Wiki</a>
</div>

<div align="center">
  <sub>A material you color generation tool for linux
</div>

<div align="center">
     <br>
     <a href="https://pypi.org/project/matugen/">
          <img alt="PyPI" src="https://img.shields.io/pypi/v/matugen?color=white&logo=pypi&logoColor=white&style=for-the-badge">
     </a>
     <a href="https://github.com/InioX/Matugen/actions/workflows/python-app.yml">
          <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/InioX/matugen/python-app.yml?color=white&style=for-the-badge">
     </a>
     <a href="https://github.com/InioX/matugen/tags/">
          <img alt="GitHub tag (latest by date)" src="https://img.shields.io/github/v/tag/InioX/matugen?color=white&logo=github&logoColor=white&style=for-the-badge">
     </a>
</div>

## Description
Matugen generates a colorscheme either from an image or a color, and exports it to a file from a template.

[Material Design 3](https://m3.material.io/) offers a new color system that allows for more flexible and dynamic use of color. The new system includes a wider range of colors, as well as a range of tints and shades that can be used to create subtle variations in color.

## Supported platforms
- Windows
- Linux
- MacOS

## Installation
### Manually compiling
>**Note** Assuming you have cargo installed
```shell
git clone https://github.com/InioX/matugen && cd matugen

TODO...
```

## Usage
```shell
# Dark theme
matugen /path/to/wallpaper/
# Light theme
matugen /path/to/wallpaper/ -l
```
Example:
```shell
matugen ~/wall/snow.png -l
```

### Creating templates
The basic syntax for using colors is `@{color}`.

There are multiple formats you can use:
```css
@define-color primary @{primary}; /* Result: ffb783 */
@define-color primary @{primary.hex}; /* Result: #ffb783 */
@define-color primary @{primary.rgb}; /* Result: rgb(255, 183, 131) */
@define-color primary @{primary.r}; /* Result: 255 */
@define-color primary @{primary.g}; /* Result: 83 */
@define-color primary @{primary.b}; /* Result: 131 */

```

You can also get the wallpaper by using:
```css
@import url("@{wallpaper}"); /* Result: /path/to/wallpaper/ */
```

Example of all the colors:
```css
/*colors.css*/
@define-color primary @{primary};
@define-color onPrimary @{onPrimary};
@define-color primaryContainer @{primaryContainer};
@define-color onPrimaryContainer @{onPrimaryContainer};
@define-color secondary @{secondary};
@define-color onSecondary @{onSecondary};
@define-color secondaryContainer @{secondaryContainer};
@define-color onSecondaryContainer @{onSecondaryContainer};
@define-color tertiary @{tertiary};
@define-color onTertiary @{onTertiary};
@define-color tertiaryContainer @{tertiaryContainer};
@define-color onTertiaryContainer @{onTertiaryContainer};
@define-color error @{error};
@define-color onError @{onError};
@define-color errorContainer @{errorContainer};
@define-color onErrorContainer @{onErrorContainer};
@define-color background @{background};
@define-color onBackground @{onBackground};
@define-color surface @{surface};
@define-color onSurface @{onSurface};
@define-color surfaceVariant @{surfaceVariant};
@define-color onSurfaceVariant @{onSurfaceVariant};
@define-color outline @{outline};
@define-color shadow @{shadow};
@define-color inverseSurface @{inverseSurface};
@define-color inverseOnSurface @{inverseOnSurface};
@define-color inversePrimary @{inversePrimary};
```

## Configuration
Here is the list of the configuration directory for different platforms:
- Windows: `C:\Users\user\AppData\Roaming\InioX\matugen\config`
- Linux: `/home/user/.config/matugen`
- MacOS: `/Users/user/Library/Application Support/com.InioX.matugen`


### Adding templates
```toml
# config_directory/config.toml

[templates.test] # First way of adding template
input_path = '~/.config/example'
output_path = '~/.config/example2'

[templates] # Second way
testts = { input_path = '~/.config/example', output_path = '~/.config/example2' }
```

## Showcase
Showcase with Hyprland, Waybar, kitty, and fish shell:

>**Warning**
>The preview and usage may be outdated.

[![](https://markdown-videos.deta.dev/youtube/rMxoORO41rs)](https://youtu.be/rMxoORO41rs)
