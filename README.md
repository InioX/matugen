# Matugen
A material you color generation tool for linux

## Contents
- [Installation](#installation)
- [Usage](#usage)
  - [Creating templates](#creating-templates)
- [Configuration](#configuration)
  - [Using Templates](#using-templates)

## Installation
TODO

## Usage
Assuming you are in the root of this project
```shell
# Dark theme
~/.local/share/pypoetry/venv/bin/poetry run python3 matugen/main.py /path/to/wallpaper/
# Light theme
~/.local/share/pypoetry/venv/bin/poetry run python3 matugen/main.py /path/to/wallpaper/ -l
```
Example:
```shell
~/.local/share/pypoetry/venv/bin/poetry run python3 matugen/main.py ~/wall/snow.png -l
```

### Creating templates
The basic syntax for color is `${color}`.

Here is a list of all the colors you can use:
- primary
- onPrimary
- primaryContainer
- onPrimaryContainer 
- secondary
- onSecondary
- secondaryContainer 
- onSecondaryContainer
- tertiary
- onTertiary
- tertiaryContainer
- onTertiaryContainer
- error
- onError
- errorContainer
- onErrorContainer
- background
- onBackground
- surface
- onSurface
- surfaceVariant
- onSurfaceVariant
- outline
- shadow
- inverseSurface
- inverseOnSurface
- inversePrimary

Example:
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

### Using templates

```ini
[Name]
template_path = relative/path/from/home/directory
output_path = relative/path/from/home/directory
```
Example:
```ini
# config.ini
[waybar]
template_path = proj/test/templates/colors.css
output_path = .config/hypr/waybar/colors.css

[rofi]
template_path = proj/test/templates/colors.rasi
output_path = .config/hypr/rofi/themes/colors.rasi

[GTK]
template_path = proj/test/templates/gtk.css
output_path = .config/gtk-4.0/gtk.css
```
