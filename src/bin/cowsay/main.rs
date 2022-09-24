use cowsay::OptionsBuilder;
use itertools::join;
use std::io::BufWriter;

mod args;

fn main() {
    let args = dbg!(args::parse());

    // TODO: -r, -l
    let opts = OptionsBuilder::default()
        .word_wrap(!args.disable_wrap)
        .print_width(args.max_width)
        // FIXME: handle this in arg parsing/validation
        .eyes([
            args.eye_string.chars().nth(0).unwrap(),
            args.eye_string.chars().nth(1).unwrap(),
        ])
        // FIXME: handle this in arg parsing/validation
        .tongue(args.tongue_string)
        .thought(args.think)
        .template(load_cowfile(&args.cowfile).unwrap())
        .filename("<figure this out>".to_string())
        .text(join(args.text, ""))
        .build()
        .unwrap();

    let mut stdout = std::io::stdout().lock();
    let mut writer = BufWriter::new(&mut stdout);
    cowsay::format(&mut writer, &opts).unwrap();
}

fn load_cowfile(name: &str) -> Result<String, &'static str> {
    // FIXME: determine template/filename from path or name
    todo!()
}
