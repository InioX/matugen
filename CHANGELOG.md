# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.4](https://github.com/InioX/Matugen/compare/matugen-v0.8.3...matugen-v0.8.4) - 2023-08-05

### Other
- update cargo and flake lockfiles

## [0.8.3](https://github.com/InioX/Matugen/compare/v0.8.2...v0.8.3) - 2023-08-05

### Added
- add sample image to assets/
- *(tapes)* add colorscheme tape
- *(tapes)* add more color formats to color.tape
- replace most unwrap()'s with expect
- add hex, rgb, hsl as color arguments
- *(template)* add lightness
- *(template)* add hue, saturation for keywords
- add triadic and adjacent color palettes
- *(wallpaper)* add feh
- *(wallpaper)* add nitrogen
- add amoled/"pure dark" mode ([#2](https://github.com/InioX/Matugen/pull/2))
- add vhs tape gifs
- add vhs tapes
- add hex code to show_colors
- add custom config file flag
- add example
- add run_after
- add waybar to reload_apps_linux
- remove .vscode
- remove result/ folder
- add result/
- add description to flake
- add flake
- add cargo.lock
- warn when wallpaper tool is not set
- add reload_gtk_theme
- add target_os for linux
- add swww options
- add set_wallpaper
- add reload gtk theme
- add app reloading for linux
- remove unused dependencies
- use if let instead of match for image
- add image to replacements
- remove image.jpg
- *(logging)* update the resizing text
- check for string length in source color
- *(logging)* update template warn style
- add .vscode/ folder
- *(logging)* use paris
- initial commit

### Fixed
- *(tapes)* change the sizes
- *(tapes)* replace image with hsl in colorscheme.tape
- *(colorscheme)* change output path
- *(tapes)* make the colorscheme tape have the same colors
- *(tapes)* update commands usage
- remove debug print statements
- *(tapes)* fix color tape size
- *(template)* fix rgba replacement
- *(template)* unclosed bracket in hex regex
- *(template)* ".hex" not working ([#3](https://github.com/InioX/Matugen/pull/3))
- swap green and blue channels ([#1](https://github.com/InioX/Matugen/pull/1) [#3](https://github.com/InioX/Matugen/pull/3))
- run_after not working
- gtk theme reload
- score sometimes choosing the wrong color
- *(logging)* show mode in reload_gtk_theme
- fix text at the end of generated template
- should fix random mess at the end of file
- remove old python folder

### Other
- *(configuration items)* fix the types
- *(usage)* add weird output note
- *(usage)* update color command usage
- *(tapes)* update help and image tapes
- *(tapes)* update every tape
- add release-plz
- update roadmap
- format with cargofmt
- *(template)* use single regex for all formats
- Revert "feat(template): add hue, saturation for keywords"
- Revert "build: add test.css and test_replaced.css"
- Revert "fix(template): fix rgba replacement"
- Revert "feat(template): add lightness"
- add test.css and test_replaced.css
- *(core palette)* change "angle" from parameter to variable ([#5](https://github.com/InioX/Matugen/pull/5))
- *(core palette)* add angle to from_hue_and_chroma ([#5](https://github.com/InioX/Matugen/pull/5))
- *(roadmap)* mark feh and nitrogen as done
- *(usage)* add amoled mode
- *(templates)* add a new keyword
- *(configuration)* add a table of all configuration items
- *(configuration)* add feh_options
- add material-color-utilities-rs
- use local material-color-utilities-rs
- Add 'material-color-utilities-rs/' from commit 'e4ebca1b8f264023ebafbcea2de94c0c17397f1e'
- update to 0.8.3
- *(roadmap)* add gtk4 ui
- *(roadmap)* add more features
- *(configuration)* fix the wording
- *(usage)* update help
- *(usage)* add gifs
- add roadmap
- add other projects
- *(showcase)* update text
- *(configuration)* add run_after
- update showcase
- update color.strip result
- update to 0.6.1
- format with cargofmt
- split read_config into multiple functions
- update flake input
- update link
- add badges
- fix license
- add license
- update to 0.4.0
- add exclude
- add installation
- add more info to package
- update to 0.3.0
- update to 0.2.2
- format with cargofmt
- update stuff
- update to 0.2.0
- update to 0.1.3
- add reload_gtk_theme
- format with cargo fmt
- add sww options to configuration
- add rgba format usage
- fix comment in image result
- update usage
- remove old buttons
- rename structs
- format code
- remove dead code
- change image syntax and result
- change installation
- rename Template::new() to generate()
- add configuration
- organise code into separate functions
- define new dimensions in a cleaner way
- rename _config to config
