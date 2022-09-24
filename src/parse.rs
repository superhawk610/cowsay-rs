use ariadne::{Label, Report, ReportKind, Source};
use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Token {
    Comment(String),
    Text(String),
    Thoughts,
    Tongue,
    Eye(u8),
}

pub fn parse(filename: &str, template: &str) -> Result<Vec<Token>, ()> {
    return match template_extractor()
        .parse(template)
        .and_then(|t| tokenizer().parse(t))
    {
        Ok(tokens) => Ok(tokens),
        Err(errors) => {
            let mut report =
                Report::build(ReportKind::Error, filename, 0).with_message("Error parsing cowfile");
            report.add_labels(errors.iter().map(|error| {
                Label::new((filename, error.span())).with_message(format!("{}", error))
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
    choice((
        just("$thoughts").to(Token::Thoughts),
        just("${thoughts}").to(Token::Thoughts),
        just("$tongue").to(Token::Tongue),
        just("${tongue}").to(Token::Tongue),
        just("$eye")
            .then(just("s").or_not())
            .map(|(_, plural)| plural.map(|_| Token::Eye(2)).unwrap_or(Token::Eye(1))),
        just("${eye")
            .then(just("s").or_not())
            .then_ignore(just("}"))
            .map(|(_, plural)| plural.map(|_| Token::Eye(2)).unwrap_or(Token::Eye(1))),
        just("\\\\").to(Token::Text("\\".to_string())),
        just("\\@").to(Token::Text("@".to_string())),
        just("\\$").to(Token::Text("$".to_string())),
        just('\\').to(Token::Text("\\".to_string())),
        just('$').to(Token::Text("$".to_string())),
        // if no keywords match, continue iterating until encountering a character
        // that _might_ start a keyword, or end the input; this means that the output
        // may contain runs of `Token::Text` delimited by `$` or `\`, but that's fine
        none_of("$\\")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(Token::Text),
    ))
    .repeated()
    .then_ignore(end())
}

fn template_extractor() -> impl Parser<char, String, Error = Simple<char>> {
    let prelude = just("$the_cow")
        .padded()
        .then(just('=').padded().or_not())
        .then(just("<<").padded())
        .then(
            just('"')
                .or_not()
                .then(just("EOC"))
                .then(just('"').or_not())
                .then(one_of(" \t").or_not()),
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

    empty()
        .then(ignored_line.repeated())
        .then(prelude)
        .ignore_then(
            take_until(just("EOC").then(text::newline()))
                .map(|(i, _)| i)
                .collect::<String>(),
        )
        .then_ignore(ignored_line.repeated())
        .then_ignore(end())
}
