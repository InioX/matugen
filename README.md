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
Assuming you are in the root of this project:
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

## Showcase
Showcase with Hyprland, Waybar, kitty, and fish shell:

[![](https://markdown-videos.deta.dev/youtube/rMxoORO41rs)](https://youtu.be/rMxoORO41rs)
