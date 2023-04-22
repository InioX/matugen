from matugen.util import (
    Theme,
    Scheme,
    Config,
    set_wallpaper,
    reload_apps,
    parse_arguments,
    get_scheme,
)


def main():
    args = parse_arguments()
    scheme = get_scheme(args)
    conf = Config.read(args.config)

    Config.generate(scheme, conf, args.wallpaper)
    reload_apps()
    set_wallpaper(args.wallpaper)


if __name__ == "__main__":
    main()
