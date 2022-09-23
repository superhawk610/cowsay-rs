use ariadne::{Label, Report, ReportKind, Source};
use chumsky::prelude::*;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let template = std::fs::read_to_string(&filename).unwrap();

    let mut stdout = std::io::stdout().lock();
    format(
        &mut stdout,
        &filename,
        &template,
        "Hello, world! This is a longer prompt to see if word-wrapping is working correctly.",
    )
    .unwrap();
}

#[derive(Debug, Clone)]
enum Token {
    Comment(String),
    Text(String),
    Thoughts,
    Tongue,
    Eye(u8),
}

// TODO: make this configurable
const THOUGHT: &'static str = "\\";
const TONGUE: &'static str = "  ";
const LEFT_EYE: &'static str = "o";
const RIGHT_EYE: &'static str = "o";
const PRINT_WIDTH: usize = 40;

#[derive(Clone, Copy)]
enum SpeechMode {
    Say,
    Think,
}

fn bubble_top<W>(out: &mut W, width: usize) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    write!(out, " ")?;
    for _ in 0..(width + 2) {
        write!(out, "_")?;
    }
    write!(out, "\n")?;
    Ok(())
}

fn bubble_bottom<W>(out: &mut W, width: usize) -> Result<(), std::io::Error>
where
    W: std::io::Write,
{
    write!(out, " ")?;
    for _ in 0..(width + 2) {
        write!(out, "-")?;
    }
    write!(out, "\n")?;
    Ok(())
}

fn format<W>(
    out: &mut W,
    filename: &str,
    template: &str,
    text: &str,
    // TODO: better error handling
) -> Result<(), Box<dyn std::error::Error>>
where
    W: std::io::Write,
{
    let lines = textwrap::wrap(text, PRINT_WIDTH);
    let width = lines
        .iter()
        .map(|s| s.chars().count())
        .max()
        .expect("always at least 1 line");
    let single_line = lines.len() == 1;
    bubble_top(out, width)?;
    for (index, line) in lines.iter().enumerate() {
        // TODO: make this configurable - thoughts are always ( / )
        let (bubble_left, bubble_right) = if single_line {
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
    for token in parse(filename, template).map_err(|_| "failed to parse")? {
        // TODO: buffer writes?
        match token {
            Token::Comment(_) => {}
            Token::Text(text) => write!(out, "{}", text)?,
            Token::Thoughts => write!(out, "{}", THOUGHT)?,
            Token::Tongue => write!(out, "{}", TONGUE)?,
            Token::Eye(mut n) => loop {
                write!(out, "{}", if left_eye { LEFT_EYE } else { RIGHT_EYE })?;
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

fn parse(filename: &str, template: &str) -> Result<Vec<Token>, ()> {
    return match tokenizer().parse(template) {
        Ok(tokens) => Ok(tokens),
        Err(errors) => {
            let mut report =
                Report::build(ReportKind::Error, filename, 0).with_message("Error parsing cowfile");
            report.add_labels(errors.iter().map(|error| {
                Label::new((filename, error.span())).with_message(format!(
                    "{:?} / expected {:?} / found {:?}",
                    error.reason(),
                    error.expected().map(|c| *c).collect::<Option<String>>(),
                    error.found()
                ))
            }));
            report
                .finish()
                .eprint((filename, Source::from(template)))
                .unwrap();

            Err(())
        }
    };
}

fn tokenizer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let space_or_tab = filter(|c: &char| *c == ' ' || *c == '\t');

    let prelude = just("$the_cow")
        .padded()
        .then(just('=').padded())
        .then(just("<<").padded())
        .then(
            just('"')
                .or_not()
                .then(just("EOC"))
                .then(just('"').or_not())
                .then(space_or_tab.or_not()),
        )
        .then(just(";").or_not())
        .then(text::newline());

    let comment = just('#')
        .then(
            take_until(text::newline())
                .map(|(i, _)| i)
                .collect::<String>()
                .map(Token::Comment),
        )
        .map(|(_, comment)| comment);

    let ignored_line = choice((text::newline(), comment.ignored()));

    let cow_part = choice((
        just("$thoughts").to(Token::Thoughts),
        just("$tongue").to(Token::Tongue),
        just("$eye")
            .then(just("s").or_not())
            .map(|(_, plural)| plural.map(|_| Token::Eye(2)).unwrap_or(Token::Eye(1))),
        just("\\\\").to(Token::Text("\\".to_string())),
        just("\\@").to(Token::Text("@".to_string())),
        just("\\$").to(Token::Text("$".to_string())),
        // if no keywords match, continue iterating until encountering a character
        // that _might_ start a keyword, or end the input; this means that the output
        // may contain runs of `Token::Text` delimited by `$` or `E`, but that's fine
        filter(|c: &char| *c != '$' && *c != 'E' && *c != '\\')
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(Token::Text),
    ));

    empty()
        .then(ignored_line.repeated())
        .then(prelude)
        .ignore_then(
            cow_part
                .repeated()
                .then_ignore(just("EOC").then(text::newline())),
        )
        .then_ignore(ignored_line.repeated())
        .then_ignore(end())
}
