<div align="center">
     <img src="https://user-images.githubusercontent.com/81521595/226138807-db504bdf-4eb5-4fe9-9ee5-a1a1395d70dc.png" width=140>
      <h1>Matugen</h1>
 </div>
    
<div align="center">
  A material you color generation tool for linux
</div>

<div align="center">
    <a href="#installation">Installation</a>
    ·
    <a href="#usage">Usage</a>
    ·
    <a href="https://github.com/InioX/matugen/wiki">Wiki</a>
</div>

## Installation
Assuming you already have [Poetry](https://python-poetry.org/) installed:
```shell
git clone https://github.com/InioX/matugen && cd matugen
poetry install
```

## Usage
Assuming you are in the root of this project
```shell
# Dark theme
poetry run python matugen/main.py /path/to/wallpaper/
# Light theme
poetry run python matugen/main.py /path/to/wallpaper/ -l
```
Example:
```shell
poetry run python matugen/main.py ~/wall/snow.png -l
```

### Creating templates
The basic syntax for using colors is `${color}`.

<details>
  <summary>Here is a list of all the colors you can use:</summary>
  <p>
    
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
    
  </p>
</details>

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
