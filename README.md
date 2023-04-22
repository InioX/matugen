<div align="center">
     <img src="https://user-images.githubusercontent.com/81521595/226138807-db504bdf-4eb5-4fe9-9ee5-a1a1395d70dc.png" width=140>
      <h1>Matugen</h1>
 </div>
    
<div align="center">
  <b>A material you color generation tool for linux</b>
</div>

<div align="center">
    <a href="#installation">Installation</a>
    ·
    <a href="#usage">Usage</a>
    ·
    <a href="https://github.com/InioX/matugen/wiki">Wiki</a>
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
[Material Design 3](https://m3.material.io/) offers a new color system that allows for more flexible and dynamic use of color. The new system includes a wider range of colors, as well as a range of tints and shades that can be used to create subtle variations in color.

## Installation
### From Pypi
>**Note** Assuming you have python with pip installed
```shell
pip install matugen
```

### Usage
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

### From repo with poetry
>**Note** Assuming you already have [Poetry](https://python-poetry.org/) installed:
```shell
git clone https://github.com/InioX/matugen && cd matugen
poetry install
```

#### Usage
```shell
# Dark theme
poetry run matugen /path/to/wallpaper/
# Light theme
poetry run matugen /path/to/wallpaper/ -l
```
Example:
```shell
poetry run matugen ~/wall/snow.png -l
```

## Showcase
Showcase with Hyprland, Waybar, kitty, and fish shell:

>**Warning**
>The preview and usage may be outdated.

[![](https://markdown-videos.deta.dev/youtube/rMxoORO41rs)](https://youtu.be/rMxoORO41rs)
