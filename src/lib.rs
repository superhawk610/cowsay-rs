use derive_builder::Builder;
use parse::Token;
use std::borrow::Cow;
use std::io::Write;

mod parse;

#[derive(Builder)]
pub struct Options {
    /// Whether word-wrapping is enabled
    word_wrap: bool,

    /// Number of columns to allow before word-wrapping
    print_width: usize,

    /// Appearance of the eyes
    eyes: [char; 2],

    /// Appearance of the tongue (2 characters)
    tongue: String,

    /// Character to use when drawing line between cow and bubble
    thought: bool,

    /// Cow template to use when rendering
    template: String,

    /// Text for cow to think/speak
    text: String,
}

fn bubble_top<W: Write>(out: &mut W, width: usize) -> Result<(), std::io::Error> {
    write!(out, " ")?;
    for _ in 0..(width + 2) {
        write!(out, "_")?;
    }
    write!(out, "\n")?;
    Ok(())
}

fn bubble_bottom<W: Write>(out: &mut W, width: usize) -> Result<(), std::io::Error> {
    write!(out, " ")?;
    for _ in 0..(width + 2) {
        write!(out, "-")?;
    }
    write!(out, "\n")?;
    Ok(())
}

// TODO: better error handling
pub fn format<W: Write>(out: &mut W, opts: &Options) -> Result<(), Box<dyn std::error::Error>> {
    let lines = if opts.word_wrap {
        textwrap::wrap(&opts.text, opts.print_width)
    } else {
        vec![Cow::Borrowed(opts.text.as_ref())]
    };
    let width = lines
        .iter()
        .map(|s| s.chars().count())
        .max()
        .expect("always at least 1 line");
    let single_line = lines.len() == 1;

    bubble_top(out, width)?;
    for (index, line) in lines.iter().enumerate() {
        let (bubble_left, bubble_right) = if opts.thought {
            ('(', ')')
        } else if single_line {
            ('<', '>')
        } else if index == 0 {
            ('/', '\\')
        } else if index == lines.len() - 1 {
            ('\\', '/')
        } else {
            ('|', '|')
        };
        write!(out, "{} ", bubble_left)?;
        write!(out, "{}", line)?;
        for _ in 0..(width - line.chars().count()) {
            write!(out, " ")?;
        }
        write!(out, " {}\n", bubble_right)?;
    }
    bubble_bottom(out, width)?;

    let mut left_eye = true;
    for token in parse::parse(&opts.template).map_err(|_| "failed to parse")? {
        match token {
            Token::Comment(_) => {}
            Token::Text(text) => write!(out, "{}", text)?,
            Token::Thoughts => write!(out, "{}", if opts.thought { 'o' } else { '\\' })?,
            Token::Tongue => write!(out, "{}", opts.tongue)?,
            Token::Eye(mut n) => loop {
                write!(
                    out,
                    "{}",
                    if left_eye { opts.eyes[0] } else { opts.eyes[1] }
                )?;
                left_eye = !left_eye;
                n -= 1;
                if n == 0 {
                    break;
                }
            },
        }
    }

    Ok(())
}
