from matugen.util import (
    Theme,
    Scheme,
    Config,
    set_wallpaper,
    reload_apps,
    parse_arguments
)

def get_scheme(args):
    scheme = Scheme(Theme.get(args.wallpaper), args.lightmode)
    return scheme.to_hex()


def main():
    CONFIG_PATH = "~/.config/matugen/config.ini"
    args = parse_arguments()
    scheme = get_scheme(args)
    conf = Config.read(CONFIG_PATH)

    Config.generate(scheme, conf, args.wallpaper)
    reload_apps()
    set_wallpaper(args.wallpaper)


if __name__ == "__main__":
    main()
