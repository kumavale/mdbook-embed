use clap::{Arg, ArgMatches, Command, crate_name, crate_version};
use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext},
};
use regex::Regex;
use std::{io, process};

pub fn make_app() -> Command {
    Command::new(crate_name!())
        .about("A mdbook preprocessor which import embed anything")
        .version(crate_version!())
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    // Users will want to construct their own preprocessor here
    let preprocessor = Embed::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

const CLASS_YOUTUBE: &str = "mdbook-embed-youtube";

struct Embed;

impl Embed {
    fn new() -> Embed {
        Embed
    }
}

impl Preprocessor for Embed {
    fn name(&self) -> &str {
        "embed-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let embed_re = Regex::new(r".*\{\{\s*#embed\s*(?P<url>.*)\s*\}\}").unwrap();
        let youtube_re = Regex::new(r".+youtube\.com.+v=(.*)").unwrap();
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chap) = item {
                chap.content = embed_re.replace_all(&chap.content, |caps: &regex::Captures| {
                    let url = caps.name("url").unwrap().as_str().to_owned();
                    if let Some(cap) = youtube_re.captures_iter(&url).next() {
                        format!("<iframe class=\"{CLASS_YOUTUBE}\" width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/{}\"></iframe>", &cap[1])
                    } else {
                        format!("<a href=\"{url}\">{url}</a>")
                    }
                }).to_string();
            }
        });
        Ok(book)
    }
}
