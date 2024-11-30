# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.5.0](https://github.com/InioX/matugen/compare/v2.4.1...v2.5.0) - 2024-11-30

### Added

- add palettes to `dump_json`

### Other

- *(readme)* improve discord link

## [2.4.1](https://github.com/InioX/matugen/compare/v2.4.0...v2.4.1) - 2024-11-13

### Fixed

- remove `wallpaper.set` option
- add `dump-json` to default featues

### Other

- run cargo fmt
- *(readme)* add discord server link

## [2.4.0](https://github.com/InioX/matugen/compare/v2.3.0...v2.4.0) - 2024-11-03

### Added

- add `mode` keyword
- *(filter)* add auto_lightness filter
- *(filter)* add camel_case filter
- custom expr and block prefix, postfix
- feature-gated web-image
- feature-gated dumping json
- feature-gated update-informer
- *(filters)* add `invert`, `grayscale and `set_hue`
- add timestamp to debug logs
- add more info to debug mode
- add `pre_hook` and `post_hook` ([#100](https://github.com/InioX/matugen/pull/100))
- improve error message for color parsing
- change resize filter to Lanczos3 ([#89](https://github.com/InioX/matugen/pull/89))
- increase windows stack size to 8mb (fixes [#87](https://github.com/InioX/matugen/pull/87))
- fix relative paths for templates, format `compare_to` ([#83](https://github.com/InioX/matugen/pull/83))
- add template formatting for hook ([#83](https://github.com/InioX/matugen/pull/83))
- add `hook` and variables inside it ([#83](https://github.com/InioX/matugen/pull/83))
- add color comparsion ([#83](https://github.com/InioX/matugen/pull/83))
- add `--prefix` argument
- add `version_check` setting ([#78](https://github.com/InioX/matugen/pull/78))

### Fixed

- `--help` flag not recognized ([#112](https://github.com/InioX/matugen/pull/112))
- parse color bug for rgb ([#107](https://github.com/InioX/matugen/pull/107))
- *(nix)* add dump-json feature build flag
- dump_json BTreeSet index
- apply more aggressive clippy lints
- removed deprecated default_features
- removed unused dependency
- cargo fmt & alejandra (nix formatter)
- made unix version compile
- wrong display of alpha channel for `set_alpha` ([#95](https://github.com/InioX/matugen/pull/95))
- divide all alpha values by 255 for output ([#95](https://github.com/InioX/matugen/pull/95))
- make hooks not depend on `colors_to_compare` ([#93](https://github.com/InioX/matugen/pull/93))
- remove useless debugging
- update arguments to remove borrow error ([#85](https://github.com/InioX/matugen/pull/85))

### Other

- run `cargo fmt`
- made contrast configurable in nix module
- make nix module able to generate from color or wallpaper.
- add backup config option to add in anything that isn't explicitly supported
- add custom colors option to nix module.
- format code
- add criterion bench
- move some stuff into template_util (prepare for criterion)
- add schemes_eq test
- use `BTreeSet`, remove `ahash` and `IndexMap`
- move template into src/
- Create rustfmt.yml
- update dependencies with breaking changes
- sorted dependencies
- removed unused dependency features
- made enquote only required for builds targeting macOS
- moved scheme out of module with only 1 file
- cargo update
- removed dependency proper-path-tools
- move some stuff into lib instead
- remove useless stuff, add clippy rules
- run cargo fmt + clippy fix
- oops bad merge ([#95](https://github.com/InioX/matugen/pull/95))
- Merge branch 'main' of https://github.com/InioX/matugen
- add float parameter for `format_hsla` and `format_rgba` ([#95](https://github.com/InioX/matugen/pull/95))
- update CHANGELOG.md
- add the set_alpha filter to the engine
- add format_rgba_float and format_hsla_float functions to format the alpha value as a float instead of u8
- add set_alpha filter
- Nix module: add package option
- bump `material-colors` to 0.4.0
- rename `compared_color` to `closest_color` ([#83](https://github.com/InioX/matugen/pull/83))
- separate some stuff into functions
- format code
- run `cargo fmt`
- *(readme)* update version badges

### Added
- add `set_alpha` filter


## [2.3.0](https://github.com/InioX/matugen/compare/v2.2.0...v2.3.0) - 2024-05-29

### Added
- rework harmonized colors into custom colors

### Fixed
- nixos flake compile error
- update `material-colors` to 0.3.1 ([#69](https://github.com/InioX/matugen/pull/69))

### Other
- Merge pull request [#73](https://github.com/InioX/matugen/pull/73) from vt-d/patch-1
- Fix issue [#71](https://github.com/InioX/matugen/pull/71)
- update material-colors to 0.3.2
- update material-colors
- support x86_64-linux + aarch64-linux by default
- make supportedSystems overridable

## [2.2.0](https://github.com/InioX/matugen/compare/v2.1.0...v2.2.0) - 2024-03-26

### Added
- add `harmonized_colors` to `--json` flag ([#53](https://github.com/InioX/matugen/pull/53))
- add `to_upper` and `to_lower` filters
- add `replace` filter
- add `set_lightness` filter
- add `colors_to_harmonize`
- *(config)* change `custom_keywords` configuration syntax

### Fixed
- move aur publish into `aur.yml`
- remove aur action in `main.yml`
- format `harmonized_colors` in `dump_json` ([#53](https://github.com/InioX/matugen/pull/53))

### Other
- *(readme)* update features
- *(readme)* add arch install guide
- Merge pull request [#58](https://github.com/InioX/matugen/pull/58) from Ehllay/main
- Merge pull request [#57](https://github.com/InioX/matugen/pull/57) from InioX/dev
- remove error message when compiling template
- run `clippy fix`

## [2.1.0](https://github.com/InioX/matugen/compare/v2.0.0...v2.1.0) - 2024-02-03

### Added
- add --contrast flag

### Fixed
- add back source_color to `--show-colors`
- use IndexMap for `--show-colors` table

### Other
- Merge pull request [#52](https://github.com/InioX/matugen/pull/52) from InioX/dev
- *(readme)* update acknowledgements
- *(readme)* remove roadmap
- bump material-colors ver

## [2.0.0](https://github.com/InioX/matugen/compare/v1.2.1...v2.0.0) - 2024-02-01

### Added
- bump material_colors ver
- add --type argument
- remove all android colors
- change config paths to relative
- add image fetched from web
- show template path and name in error
- add template name and path in error
- update syntax
- add span of file in template render error
- add custom keywords
- add new formats for keywords
- replace regex with `upon`
- update example config and template file
- add `--debug` flag
- remove `run_after`
- *(macos)* implement wallpaper setting
- *(template)* show error if file is not in UTF-8
- *(wallpaper)* add error is the program to set wallpaper is not in PATH
- *(windows)* implement setting wallpaper
- add `update-informer`
- *(logging)* add number indicator to logs for templates and run_after
- *(logging)* update the message format
- *(COLORS_ANDROID)* add `source_color`
- *(template)* add `hsl` and `hsla` formats
- *(template)* add `dark`, `amoled` and `light` schemes to every template
- *(scheme)* add `android_scheme` keywords
- *(scheme)* add `android_scheme`
- *(arguments)* add `--show-colors` flag and disable showing colors by default
- *(show_colors)* show light, dark and amoled at once in a table
- *(arguments)* [**breaking**] remove `lightmode` and `amoled`
- *(config)* [**breaking**] rename `scheme` to `mode` in template config
- *(arguments)* add `mode` to replace `lightmode` and `amoled`
- *(template)* add `Light`,`Dark`,`Amoled` options for each template
- add `--dry-run` flag
- *(template)* add `source_color`
- show generated colors inside a table instead
- *(reload apps)* make every app true by default
- *(reload apps)* add dunst
- *(config)* add suggestion to error message
- *(config)* [**breaking**] add reload_apps_list
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
- add amoled/"pure dark" mode ([#2](https://github.com/InioX/matugen/pull/2))
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
- STATUS_STACK_OVERFLOW when quantizing
- *(macos)* add `enquote` to global dependencies ([#43](https://github.com/InioX/matugen/pull/43))
- light/dark theme being swapped everywhere
- fix table light/dark modes
- fix nix module command
- *(macos)* change `use` to `extern` for enquote ([#43](https://github.com/InioX/matugen/pull/43))
- *(macos)* enquote not imported ([#43](https://github.com/InioX/matugen/pull/43))
- forgot to commit `Cargo.lock`
- make reqwest not use openssl
- remove openssl dependency
- update quantizer arguments [#39](https://github.com/InioX/matugen/pull/39)
- add back `source_color`
- red color being blue in generated file
- remove compilation errors on unix
- `--show-colors`, `--json` not showing without an image
- wrong hsl color in generated file
- do not open file before rendering template
- *(template)* fix "parent folder does not exist" warning
- `--quiet` flag still showing output
- make `enquote` dependency only for macos target
- `run_after` not working on windows
- fix typo in `run_after` function
- remove unused result warning
- *(macos)* update specific functions to only run on linux and macos ([#25](https://github.com/InioX/matugen/pull/25))
- *(macos)* use conditional import for unsupported modules ([#25](https://github.com/InioX/matugen/pull/25))
- *(template)* change println to debug
- *(template)* improve error handling for files and folders
- *(android_scheme)* fix `light` and `pure_dark` colors
- *(show_color)* make the dark and amoled colors use right schemes
- remove debug stuff
- dark and amoled modes being switched
- correct blue and green in `get_source_color`
- show colors AFTER running commands
- add NetBSD support
- *(release-plz)* change field name
- *(tapes)* change the sizes
- *(tapes)* replace image with hsl in colorscheme.tape
- *(colorscheme)* change output path
- *(tapes)* make the colorscheme tape have the same colors
- *(tapes)* update commands usage
- remove debug print statements
- *(tapes)* fix color tape size
- *(template)* fix rgba replacement
- *(template)* unclosed bracket in hex regex
- *(template)* ".hex" not working ([#3](https://github.com/InioX/matugen/pull/3))
- swap green and blue channels ([#1](https://github.com/InioX/matugen/pull/1) [#3](https://github.com/InioX/matugen/pull/3))
- run_after not working
- gtk theme reload
- score sometimes choosing the wrong color
- *(logging)* show mode in reload_gtk_theme
- fix text at the end of generated template
- should fix random mess at the end of file
- remove old python folder

### Other
- Merge pull request [#46](https://github.com/InioX/matugen/pull/46) from InioX/material-colors-rewrite
- remove `material-color-utilities-rs` dependency
- change colors.css
- replace `format_argb_as_rgb`
- remove leftover logging
- update nix module matugen command
- use `material-colors` instead of `material-color-utilities-rs`
- update `generate_pixels` function
- remove amoled scheme
- *(readme)* add note to nixos module
- *(readme)* update header and buttons
- update version manually
- update all dependencies
- Revert "chore(matugen): release v1.2.0"
- release
- update gitignore
- release
- Merge pull request [#35](https://github.com/InioX/matugen/pull/35) from InioX/release-plz-2023-12-14T18-36-35Z
- run cargo fix
- Revert "chore(matugen): release v1.1.0"
- release
- Merge branch 'main' of https://github.com/InioX/matugen
- update version
- clean up `generate_color_strings`
- update roadmap
- release
- clean up `Template::generate` function arguments
- run `cargo update`
- remove unneeded imports
- update roadmap and wallpaper alert
- format with `cargo fmt`
- changed function arguments to not use `config` or `args` directly
- move some files into os specific folders
- move update_informer into a function
- format with cargofmt
- release
- update note syntax
- update roadmap icon
- release
- update version
- Added new surfaces, fixed colors, removed deprecated colors, changed chroma for neutral palette
- cargo fmt
- fix module not outputting templates
- fix module error
- add NixOS/HM module
- move `usage` and `configuration` into the wiki
- add icons to headers
- release
- remove build warnings (closes [#17](https://github.com/InioX/matugen/pull/17))
- add `--json` docs
- format with cargofmt
- add --json flag
- run formatter
- update note markdown
- *(nixos)* add specific version for flake
- move some stuff into their own functions
- release
- *(template)* update keyword names
- *(`scheme_android`)* add TODO note for amoled scheme
- add DEFAULT_CONFIG const
- *(get_source_color)* move to color file
- rename `Commands` struct to `Source`
- make the colors vec a const
- add gifs for modes, palettes and `--other-colors`
- update all tapes
- *(tapes)* change the output directory
- remove unnecessary imports
- fix usage of commands
- add an explanation for `mode` in template config
- rename `scheme` to `mode`
- add `source color` keyword
- format with `cargo fmt`
- *(show_color)* use `format_argb_as_rgb`
- format with `cargo fmt`
- change repository address
- remove useless gifs
- release
- add removed notice for `reload_gtk_theme`
- run clippy --fix
- release
- update cargo and flake lockfiles
- add workspace and ini-material-color-utilities-rs
- change version and name
- add release-plz.toml
- release
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
- *(core palette)* change "angle" from parameter to variable ([#5](https://github.com/InioX/matugen/pull/5))
- *(core palette)* add angle to from_hue_and_chroma ([#5](https://github.com/InioX/matugen/pull/5))
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

## [1.1.1](https://github.com/InioX/matugen/compare/matugen-v1.1.0...matugen-v1.1.1) - 2023-12-14

### Fixed
- red color being blue in generated file

## [1.1.0](https://github.com/InioX/matugen/compare/matugen-v1.0.0...matugen-v1.1.0) - 2023-12-14

### Added
- add span of file in template render error
- add custom keywords

### Fixed
- `--show-colors`, `--json` not showing without an image
- wrong hsl color in generated file
- do not open file before rendering template

### Other
- Merge branch 'main' of https://github.com/InioX/matugen

## [0.11.2](https://github.com/InioX/matugen/compare/matugen-v0.11.1...matugen-v0.11.2) - 2023-12-03

### Added
- update example config and template file
- add `--debug` flag
- remove `run_after`
- *(macos)* implement wallpaper setting
- *(template)* show error if file is not in UTF-8
- *(wallpaper)* add error is the program to set wallpaper is not in PATH
- *(windows)* implement setting wallpaper
- add `update-informer`
- *(logging)* add number indicator to logs for templates and run_after
- *(logging)* update the message format

### Fixed
- *(template)* fix "parent folder does not exist" warning
- `--quiet` flag still showing output
- make `enquote` dependency only for macos target
- `run_after` not working on windows
- fix typo in `run_after` function

### Other
- clean up `Template::generate` function arguments
- run `cargo update`
- remove unneeded imports
- update roadmap and wallpaper alert
- format with `cargo fmt`
- changed function arguments to not use `config` or `args` directly
- move some files into os specific folders
- move update_informer into a function
- format with cargofmt

## [0.11.1](https://github.com/InioX/matugen/compare/matugen-v0.11.0...matugen-v0.11.1) - 2023-11-17

### Fixed
- remove unused result warning
- *(macos)* update specific functions to only run on linux and macos ([#25](https://github.com/InioX/matugen/pull/25))
- *(macos)* use conditional import for unsupported modules ([#25](https://github.com/InioX/matugen/pull/25))

### Other
- update note syntax
- update roadmap icon

## [0.10.1](https://github.com/InioX/matugen/compare/matugen-v0.10.0...matugen-v0.10.1) - 2023-09-17

### Added
- *(COLORS_ANDROID)* add `source_color`
- *(template)* add `hsl` and `hsla` formats
- *(template)* add `dark`, `amoled` and `light` schemes to every template

### Fixed
- *(android_scheme)* fix `light` and `pure_dark` colors

### Other
- remove build warnings (closes [#17](https://github.com/InioX/matugen/pull/17))
- add `--json` docs
- format with cargofmt
- add --json flag
- run formatter
- update note markdown
- *(nixos)* add specific version for flake
- move some stuff into their own functions

## [0.10.0](https://github.com/InioX/matugen/compare/matugen-v0.9.0...matugen-v0.10.0) - 2023-08-15

### Added
- *(scheme)* add `android_scheme` keywords
- *(scheme)* add `android_scheme`
- *(arguments)* add `--show-colors` flag and disable showing colors by default
- *(show_colors)* show light, dark and amoled at once in a table
- *(arguments)* remove `lightmode` and `amoled`
- *(config)* rename `scheme` to `mode` in template config
- *(arguments)* add `mode` to replace `lightmode` and `amoled`
- *(template)* add `Light`,`Dark`,`Amoled` options for each template
- add `--dry-run` flag
- *(template)* add `source_color`
- show generated colors inside a table instead

### Fixed
- *(show_color)* make the dark and amoled colors use right schemes
- remove debug stuff
- dark and amoled modes being switched
- correct blue and green in `get_source_color`
- show colors AFTER running commands

### Other
- *(template)* update keyword names
- *(`scheme_android`)* add TODO note for amoled scheme
- add DEFAULT_CONFIG const
- *(get_source_color)* move to color file
- rename `Commands` struct to `Source`
- make the colors vec a const
- add gifs for modes, palettes and `--other-colors`
- update all tapes
- *(tapes)* change the output directory
- remove unnecessary imports
- fix usage of commands
- add an explanation for `mode` in template config
- rename `scheme` to `mode`
- add `source color` keyword
- format with `cargo fmt`
- *(show_color)* use `format_argb_as_rgb`
- format with `cargo fmt`
- change repository address
- remove useless gifs

## [0.9.0](https://github.com/InioX/Matugen/compare/matugen-v0.8.4...matugen-v0.9.0) - 2023-08-08

### Added
- *(reload apps)* make every app true by default
- *(reload apps)* add dunst
- *(config)* add suggestion to error message
- *(config)* [**breaking**] add reload_apps_list

### Fixed
- add NetBSD support

### Other
- add removed notice for `reload_gtk_theme`
- run clippy --fix

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
