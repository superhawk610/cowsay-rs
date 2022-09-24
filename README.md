# cowsay-rs

[`cowsay(1)`](https://linux.die.net/man/1/cowsay) rewritten in Rust, a
configurable speaking/thinking cow (and a bit more).

```
$ cowsay "Hello, world!"
 _______________
< Hello, world! >
 ---------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

## Usage

```plain
USAGE:
    cowsay [-e eye_string] [-f cowfile] [-h] [-l] [-n] [-T tongue_string] [-W column] [-bdgpstwy] <text> ...

ARGS:
    <TEXT>...    Text to display (may instead read from stdin)

OPTIONS:
    -e <EYE_STRING>           Appearance of the cow's eyes (should be 2 chars) [default: oo]
    -f <COWFILE>              Path to cowfile, or name of built-in cowfile [default: default]
    -T <TONGUE_STRING>        Appearance of the cow's tongue (should be 2 chars) [default: "  "]
    -W <MAX_WIDTH>            Max width for word wrapping [default: 40]
    -b                        Mode: borg
    -d                        Mode: dead
    -g                        Mode: greedy
    -p                        Mode: paranoia
    -s                        Mode: stoned
    -t                        Mode: tired
    -w                        Mode: wired
    -y                        Mode: youthful
    -l                        List built-in cowfiles
    -n                        Disable word-wrapping
    -r                        Select a random cow
        --think               Think instead of speak
    -h, --help                Print help information
    -V, --version             Print version information
```

## Acknowledgements

Special thanks to [piuccio/cowsay](https://github.com/piuccio/cowsay) for
providing a reference implementation, along with a ton of great cowfiles (all
included with this project).

## License

&copy; 2022 Aaron Ross, All Rights Reserved.

`cowsay(1)` originally released by Tony Monroe.
