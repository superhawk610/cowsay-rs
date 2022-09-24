use cowsay::OptionsBuilder;
use include_dir::{include_dir, Dir};
use itertools::join;
use rand::seq::IteratorRandom;
use std::borrow::Cow;
use std::io::BufWriter;

mod args;

fn main() {
    let mut args = args::parse();

    if args.list {
        list_builtin_cowfiles();
        std::process::exit(0);
    }

    if args.random {
        args.cowfile = random_cowfile();
    }

    let (name, template) = load_cowfile(&args.cowfile).unwrap();

    let opts = OptionsBuilder::default()
        .word_wrap(!args.disable_wrap)
        .print_width(args.max_width)
        .eyes([
            args.eye_string.chars().nth(0).expect("valid parse"),
            args.eye_string.chars().nth(1).expect("valid parse"),
        ])
        .tongue(args.tongue_string)
        .thought(args.think)
        .filename(name)
        .template(template.to_string())
        .text(join(args.text, " "))
        .build()
        .unwrap();

    let mut stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(&mut stdout);
    cowsay::format(&mut writer, &opts).unwrap();
}

static BUILTIN_COWS: Dir<'_> = include_dir!("cows");

fn list_builtin_cowfiles() {
    for cowfile in BUILTIN_COWS.files() {
        print!(
            "{}  ",
            cowfile.path().file_stem().unwrap().to_str().unwrap()
        );
    }
    print!("\n");
}

fn random_cowfile() -> String {
    let mut rng = rand::thread_rng();
    BUILTIN_COWS
        .files()
        .choose(&mut rng)
        .map(|f| f.path())
        .unwrap()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn load_cowfile(name: &str) -> Result<(String, Cow<str>), &'static str> {
    // cowfile may specify a filesystem path
    if name.contains(std::path::MAIN_SEPARATOR) {
        let name = name.to_string();
        let contents = std::fs::read_to_string(&name)
            .map(|s| Cow::Owned(s))
            .map_err(|_| "cannot find cowfile")?;
        return Ok((name, contents));
    }

    let name = format!("{}.cow", name);
    let contents = BUILTIN_COWS
        .get_file(&name)
        .and_then(|f| f.contents_utf8())
        .map(|s| Cow::Borrowed(s))
        .ok_or("unable to read built-in cowfile")?;
    Ok((name, contents))
}
