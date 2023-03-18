from util import (
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
    args = parse_arguments()
    scheme = get_scheme(args)
    conf = Config.read("config.ini")

    Config.generate(scheme, conf, args.wallpaper)
    reload_apps()
    set_wallpaper(args.wallpaper)


if __name__ == "__main__":
    main()
