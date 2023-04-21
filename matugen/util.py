import os
import re
import logging
import pathlib
import subprocess
import importlib.metadata
from material_color_utilities_python import Image, themeFromImage
from rich.logging import RichHandler
from configparser import ConfigParser
from argparse import Namespace, ArgumentParser
from pathlib import Path


def get_version() -> str:
    return importlib.metadata.version('matugen')


def get_scheme(args):
    scheme = Scheme(Theme.get(args.wallpaper), args.lightmode)
    return scheme.to_hex()


def parse_arguments():
    parser = ArgumentParser()

    parser.add_argument(
        "wallpaper",
        help="the wallpaper for generating colorschemes",
        type=str
    )

    parser.add_argument(
        "-l", "--lightmode",
        help="whether to use light mode",
        action="store_true"
    )
    parser.add_argument(
        "--version",
        help="outputs the version",
        action="version",
        version=get_version()
    )
    parser.add_argument(
        "-c", "--config",
        help="the config for generating templates",
        default="~/.config/matugen/config.ini",
        type=str
    )
    args: Namespace = parser.parse_args()
    return args

def setup_logging():
    FORMAT = "%(message)s"
    logging.basicConfig(
        level="INFO", format=FORMAT, datefmt="[%X]", handlers=[RichHandler()]
    )

    log = logging.getLogger("rich")
    return log


log = setup_logging()


def reload_apps():
    commands = [
        ["pkill", "-SIGUSR2", "waybar"],
        ["gsettings", "set", "org.gnome.desktop.interface",
            "gtk-theme", "adw-gtk3-dark"],
        ["pkill", "-SIGUSR1", "kitty"]
    ]
    for cmd in commands:
        subprocess.run(
            cmd,
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        )

    log.info("Restarting GTK")
    os.system("gsettings set org.gnome.desktop.interface gtk-theme adw-gtk3-dark > /dev/null 2>&1")

    log.info("Restarting kitty")
    os.system("pkill -SIGUSR1 kitty")


def set_wallpaper(path: str):
    log.info("Setting wallpaper with swaybg")
    os.system("pkill swaybg > /dev/null 2>&1")
    os.system(f"swaybg -i {path} > /dev/null 2>&1 &")


class Color:
    @staticmethod
    def rgb_to_hex(rgb: int) -> str:
        return '%02x%02x%02x' % rgb

    def hex_to_rgb(hexa: str) -> tuple[int, int, int]:
        return tuple(int(hexa[i:i+2], 16) for i in (0, 2, 4))

    @staticmethod
    def dec_to_rgb(dec: int) -> int:
        red = (dec >> 16) & 255
        green = (dec >> 8) & 255
        blue = dec & 255

        return red, green, blue


class Config:
    @staticmethod
    def read(filename: str):
        config = ConfigParser()
        config_path = Path(filename).expanduser()
        try:
            config.read(config_path)
        except OSError as err:
            logging.exception(f"Could not open {err.filename}")
        else:
            logging.info(
                f"Loaded {len(config.sections())} templates from {config_path}"
            )
            return config

    @staticmethod
    def generate(scheme: dict, config: ConfigParser, wallpaper: str) -> dict:
        for i, item in enumerate(config.sections()):
            template_name = config[item].name
            template_path = Path(config[item]["template_path"]).expanduser()
            output_path = Path(config[item]["output_path"]).expanduser()

            try:
                with open(template_path, "r") as input:  # Template file
                    input_data = input.read()
            except OSError as err:
                logging.error(f"Could not open {err.filename}")
                i += 1
                return

            output_data = input_data
            # print(f"i: {i}")
            for h, (key, value) in enumerate(scheme.items()):
                # print(f"H: {h}")
                pattern = re.compile(f"@{{{key}}}")
                pattern_hex = re.compile(f"@{{{key}.hex}}")
                pattern_rgb = re.compile(f"@{{{key}.rgb}}")
                pattern_wallpaper = re.compile("@{wallpaper}")

                hex_stripped = value[1:]
                rgb_value = f"rgb{Color.hex_to_rgb(hex_stripped)}"
                wallpaper_value = os.path.abspath(wallpaper)

                output_data = pattern.sub(hex_stripped, output_data)
                output_data = pattern_hex.sub(value, output_data)
                output_data = pattern_rgb.sub(rgb_value, output_data)
                output_data = pattern_wallpaper.sub(
                    wallpaper_value, output_data)
                i += 1

            try:
                with open(output_path, "w") as output:
                    output.write(output_data)
            except OSError as err:
                logging.exception(
                    f"Could not write {template_name} template to {err.filename}")
            else:
                log.info(
                    f"Exported {template_name} template to {output_path}")


class Theme:
    @staticmethod
    def get(image: str):
        log.info(f"Using image {image}")
        img = Image.open(image)
        basewidth = 64
        wpercent = (basewidth/float(img.size[0]))
        hsize = int((float(img.size[1])*float(wpercent)))
        img = img.resize((basewidth, hsize), Image.Resampling.LANCZOS)

        return themeFromImage(img)


class Scheme():
    def __init__(self, theme: str, lightmode: bool):
        if lightmode:
            log.info("Using light scheme")
            self.scheme_dict = theme["schemes"]["light"].props
        else:
            log.info("Using dark scheme")
            self.scheme_dict = theme["schemes"]["dark"].props

    def get(self) -> dict:
        return self.scheme_dict

    def to_rgb(self) -> dict:
        scheme = self.scheme_dict

        for key, value in scheme.items():
            scheme[key] = Color.dec_to_rgb(value)
        return scheme

    def to_hex(self) -> dict:
        scheme = self.scheme_dict

        # Need to convert to rgb first
        self.to_rgb()

        for key, value in scheme.items():
            scheme[key] = "#{value}".format(value=Color.rgb_to_hex(value))
        return scheme
