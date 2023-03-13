# Matugen
A material you color generation tool for linux

## Contents
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Templates](#templates)

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

## Configuration

### Templates

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
