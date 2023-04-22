import importlib.metadata
import logging
import os
import re
import subprocess
from argparse import ArgumentParser, Namespace
from configparser import ConfigParser
from pathlib import Path

from material_color_utilities_python import Image, themeFromImage
from rich import traceback
from rich.logging import RichHandler

traceback.install(extra_lines=3)

logging.basicConfig(
    level="INFO", format="%(message)s", datefmt="[%X]", handlers=[RichHandler()]
)

log = logging.getLogger("rich")


class InvalidFileExtension(Exception):
    """
    Exception raised when provided extension doesnt match with the expected one

    @param extension -- The extension of the file
    @param message -- The expected extension of the file
    """

    def __init__(self, expected_extension, extension):
        self.extension = extension
        self.expected_extension = expected_extension

        self.message = (
            f"Invalid file extension. Expected {expected_extension}, got '{extension}'"
        )
        super().__init__(self.message)


def get_version() -> str:
    """
    Get matugen version. This is a wrapper around importlib.metadata.version().


    @return matugen version as a string
    """
    return importlib.metadata.version("matugen")


def get_scheme(args):
    """
    Get the scheme to use. This is based on the wallpaper and lightmode passed in the command line arguments

    @param args - The command line arguments parsed by ConfigParser

    @return The generated colorscheme with colors in hex format.
    """
    scheme = Scheme(Theme.get(args.wallpaper), args.lightmode)
    return scheme.to_hex()


def parse_arguments():
    """
    Parse command line arguments.


    @return Namespace object containing the parsed command line arguments or None if there are no arguments to be parsed.
    """
    parser = ArgumentParser()

    parser.add_argument(
        "wallpaper", help="the wallpaper for generating colorschemes", type=str
    )

    parser.add_argument(
        "-l", "--lightmode", help="whether to use light mode", action="store_true"
    )
    parser.add_argument(
        "--version", help="outputs the version", action="version", version=get_version()
    )
    parser.add_argument(
        "-c",
        "--config",
        help="the config for generating templates",
        default="~/.config/matugen/config.ini",
        type=str,
    )
    args: Namespace = parser.parse_args()

    # Check if the args provided are valid
    VALID_WALLPAPER_EXTENSIONS = [".png", ".jpg"]
    VALID_CONFIG_EXTENSIONS = [".ini"]
    WALLPAPER_PATH = Path(args.wallpaper).expanduser()
    CONFIG_PATH = Path(args.config).expanduser()

    WALLPAPER_EXTENSION = Path(args.wallpaper).suffix
    CONFIG_EXTENSION = Path(args.config).suffix

    if WALLPAPER_EXTENSION == "":
        raise FileNotFoundError("Invalid wallpaper file path.")
    elif Path(args.wallpaper).suffix not in VALID_WALLPAPER_EXTENSIONS:
        raise InvalidFileExtension(VALID_WALLPAPER_EXTENSIONS, WALLPAPER_EXTENSION)
    elif not os.path.isfile(WALLPAPER_PATH):
        raise FileNotFoundError("The wallpaper file doesnt exist.")

    if CONFIG_EXTENSION == "":
        raise FileNotFoundError("Invalid config file path.")
    elif Path(args.config).suffix not in VALID_CONFIG_EXTENSIONS:
        raise InvalidFileExtension(VALID_CONFIG_EXTENSIONS, CONFIG_EXTENSION)
    elif not Path.is_file(CONFIG_PATH):
        raise FileNotFoundError("The config file doesnt exist.")

    return args


def reload_apps():
    """
    Reload apps to change the colors.
    """
    commands = [
        ["pkill", "-SIGUSR2", "waybar"],
        [
            "gsettings",
            "set",
            "org.gnome.desktop.interface",
            "gtk-theme",
            "adw-gtk3-dark",
        ],
        ["pkill", "-SIGUSR1", "kitty"],
    ]
    # Run all commands in the commands list
    for cmd in commands:
        subprocess.run(
            cmd, check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
        )


def get_session():
    """
    Get the currently logged in user's session. This is useful for knowing whether to use wayland specific programs or not.


    @return string of the session
    """
    command = "loginctl show-session $(loginctl show-user $(whoami) -p Display --value) -p Type --value"
    session = subprocess.run(command, capture_output=True, shell=True)
    return session.stdout.decode().strip()


def set_wallpaper(path: str):
    """
    Set wallpaper to given path.

    @param path - The path to the wallpaper to set.
    """
    session = get_session()

    # Setting the wallpaper on wayland with swaybg.
    if session == "wayland":
        log.info("Wayland detected, setting wallpaper with swaybg")
        os.system("pkill swaybg > /dev/null 2>&1")
        os.system(f"swaybg -i {path} > /dev/null 2>&1 &")
    else:
        # TODO Use something else for x11
        return


class Color:
    @staticmethod
    def rgb_to_hex(rgb: int) -> str:
        """
        Convert an RGB value to a hex string.

        @param rgb - The RGB value to convert. Must be between 0 and 255.

        @return The hex string representation of the RGB value.
        """
        return "%02x%02x%02x" % rgb

    def hex_to_rgb(hex: str) -> tuple[int, int, int]:
        """
        Convert hex color to RGB.

        @param hex - A string containing the hex value to convert.

        @return A tuple of RGB values.
        """
        return tuple(int(hex[i : i + 2], 16) for i in (0, 2, 4))

    @staticmethod
    def dec_to_rgb(dec: int) -> int:
        """
        Convert decimal to RGB.

        @param dec - The decimal to convert.

        @return A tuple of RGB values.
        """
        red = (dec >> 16) & 255
        green = (dec >> 8) & 255
        blue = dec & 255

        return red, green, blue


class Config:
    @staticmethod
    def read(file: str):
        """
        Read and parse config file. This is a wrapper around ConfigParser.read() that handles logging and error handling

        @param filename - Path to config file.

        @return Config object with parsed sections and template names from config file or None if there was an error reading the config.
        """
        config = ConfigParser()
        CONFIG_PATH = Path(file).expanduser()
        try:
            config.read(CONFIG_PATH)
        except OSError as err:
            logging.exception(f"Could not open {err.file}")
        else:
            logging.info(
                f"Loaded {len(config.sections())} templates from {CONFIG_PATH}"
            )
            return config

    @staticmethod
    def generate(scheme: dict, config: ConfigParser, wallpaper: str) -> dict:
        templates = [
            {
                "name": config[item].name,
                "template_path": Path(config[item]["template_path"]).expanduser(),
                "output_path": Path(config[item]["output_path"]).expanduser(),
            }
            for item in config.sections()
        ]

        HEX_VALUES = {key: value[1:] for key, value in scheme.items()}
        RGB_VALUES = {
            key: f"rgb{Color.hex_to_rgb(value[1:])}" for key, value in scheme.items()
        }

        # TODO: Use pathlib.Path instead of os.path
        WALLPAPER_PATH = os.path.abspath(wallpaper)

        for template in templates:
            try:
                with open(template["template_path"], "r") as input_file:
                    input_data = input_file.read()
            except OSError as err:
                logging.exception(f"Could not open {err.filename}")
                continue

            output_data = input_data
            for key in scheme:
                PATTERN = re.compile(f"@{{{key}}}")
                PATTERN_HEX = re.compile(f"@{{{key}.hex}}")
                PATTERN_RGB = re.compile(f"@{{{key}.rgb}}")
                PATTERN_WALLPAPER = re.compile("@{wallpaper}")

                output_data = PATTERN.sub(HEX_VALUES[key], output_data)
                output_data = PATTERN_HEX.sub(scheme[key], output_data)
                output_data = PATTERN_RGB.sub(RGB_VALUES[key], output_data)
                output_data = PATTERN_WALLPAPER.sub(WALLPAPER_PATH, output_data)

            try:
                with open(template["output_path"], "w") as output_file:
                    output_file.write(output_data)
            except OSError as err:
                logging.exception(
                    f'Could not write {template["name"]} template to {err.filename}'
                )
            else:
                logging.info(
                    f'Exported {template["name"]} template to {template["output_path"]}'
                )


class Theme:
    @staticmethod
    def get(image: str):
        """
        Get theme from image. This is a wrapper around themeFromImage that resizes the image to 64x64 for performance and returns the theme.

        @param image - Path to the image to use.

        @return The theme as a dict containing multiple colorchemes.
        """
        log.info(f"Using image {image}")
        img = Image.open(image)
        BASEWIDTH = 64
        wpercent = BASEWIDTH / float(img.size[0])
        hsize = int((float(img.size[1]) * float(wpercent)))
        img = img.resize((BASEWIDTH, hsize), Image.Resampling.LANCZOS)

        return themeFromImage(img)


class Scheme:
    def __init__(self, theme: str, lightmode: bool):
        """
        Depending on whether lightmode is true, initialize the light or dark scheme.

        @param theme - The theme to use for color scheme.
        @param lightmode - Whether to use lightmode.
        """
        # Use light mode if lightmode is enabled.
        if lightmode:
            log.info("Using light scheme")
            self.scheme_dict = theme["schemes"]["light"].props
        else:
            log.info("Using dark scheme")
            self.scheme_dict = theme["schemes"]["dark"].props

    def get(self) -> dict:
        """
        Get the scheme dict.


        @return The scheme dict in dark or light mode.
        """
        return self.scheme_dict

    def to_rgb(self) -> dict:
        """
        Convert the scheme colors to RGB.


        @return The scheme dict in RGB.
        """
        scheme = self.scheme_dict

        for key, value in scheme.items():
            scheme[key] = Color.dec_to_rgb(value)
        return scheme

    def to_hex(self) -> dict:
        """
        Convert the scheme colors to hex; first convert to RGB.


        @return The scheme dict in hex.
        """
        scheme = self.scheme_dict

        # Need to convert to rgb first
        self.to_rgb()

        for key, value in scheme.items():
            scheme[key] = "#{value}".format(value=Color.rgb_to_hex(value))
        return scheme
