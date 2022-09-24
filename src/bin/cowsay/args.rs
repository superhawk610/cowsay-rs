use clap::{CommandFactory, Parser};
use std::io::{stdin, Read};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, usage = "cowsay [-e eye_string] [-f cowfile] [-h] [-l] [-n] [-T tongue_string] [-W column] [-bdgpstwy] <text> ...")]
pub struct Args {
    /// Appearance of the cow's eyes (should be 2 chars)
    #[clap(short = 'e', default_value = "oo", display_order = 1)]
    pub eye_string: String,

    /// Path to cowfile, or name of built-in cowfile
    #[clap(short = 'f', default_value = "default", display_order = 1)]
    pub cowfile: String,

    /// List built-in cowfiles
    #[clap(short = 'l', display_order = 3)]
    pub list: bool,

    /// Disable word-wrapping
    #[clap(short = 'n', display_order = 3)]
    pub disable_wrap: bool,

    /// Appearance of the cow's tongue (should be 2 chars)
    #[clap(short = 'T', default_value = "  ", display_order = 1)]
    pub tongue_string: String,

    /// Max width for word wrapping
    #[clap(short = 'W', default_value_t = 40, display_order = 1)]
    pub max_width: usize,

    /// Select a random cow
    #[clap(short = 'r', display_order = 3)]
    pub random: bool,

    /// Think instead of speak
    #[clap(long, display_order = 4)]
    pub think: bool,

    /// Mode: borg
    #[clap(short = 'b', display_order = 2, group = "mode")]
    mode_borg: bool,

    /// Mode: dead
    #[clap(short = 'd', display_order = 2, group = "mode")]
    mode_dead: bool,

    /// Mode: greedy
    #[clap(short = 'g', display_order = 2, group = "mode")]
    mode_greedy: bool,

    /// Mode: paranoia
    #[clap(short = 'p', display_order = 2, group = "mode")]
    mode_paranoia: bool,

    /// Mode: stoned
    #[clap(short = 's', display_order = 2, group = "mode")]
    mode_stoned: bool,

    /// Mode: tired
    #[clap(short = 't', display_order = 2, group = "mode")]
    mode_tired: bool,

    /// Mode: wired
    #[clap(short = 'w', display_order = 2, group = "mode")]
    mode_wired: bool,

    /// Mode: youthful
    #[clap(short = 'y', display_order = 2, group = "mode")]
    mode_youthful: bool,

    /// Text to display (may instead read from stdin)
    pub text: Vec<String>,
}

pub fn parse() -> Args {
    let mut args = Args::parse();

    // Modes are applied in the order they're displayed in the help,
    // and take precedence over any custom eye/tongue strings.
    if args.mode_borg {
        args.eye_string = "==".to_string();
        args.tongue_string = "  ".to_string();
    } else if args.mode_dead {
        args.eye_string = "xx".to_string();
        args.tongue_string = "U ".to_string();
    } else if args.mode_greedy {
        args.eye_string = "$$".to_string();
        args.tongue_string = "  ".to_string();
    } else if args.mode_paranoia {
        args.eye_string = "@@".to_string();
        args.tongue_string = "  ".to_string();
    } else if args.mode_stoned {
        args.eye_string = "**".to_string();
        args.tongue_string = "U ".to_string();
    } else if args.mode_tired {
        args.eye_string = "--".to_string();
        args.tongue_string = "  ".to_string();
    } else if args.mode_wired {
        args.eye_string = "OO".to_string();
        args.tongue_string = "  ".to_string();
    } else if args.mode_youthful {
        args.eye_string = "..".to_string();
        args.tongue_string = "  ".to_string();
    }

    if args.tongue_string.chars().count() != 2 || args.eye_string.chars().count() != 2 {
        Args::command()
            .error(
                clap::ErrorKind::InvalidValue,
                "<tongue_string> and <eye_string> must be exactly two UTF-8 characters",
            )
            .exit();
    }

    if !args.list && args.text.is_empty() {
        // if no text is provided and the user didn't provide anything
        // on stdin, attempting to read from stdin will block the process
        // and provide the user with a TTY to enter input; we'd instead
        // rather just print the usage message and exit
        if atty::is(atty::Stream::Stdin) {
            Args::command().print_help().unwrap();
            std::process::exit(1);
        }

        let mut buf = String::new();
        stdin()
            .lock()
            .read_to_string(&mut buf)
            .expect("read from stdin");
        args.text = vec![buf.trim().to_string()];
    }

    args
}
