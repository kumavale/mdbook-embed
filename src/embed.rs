use mdbook::{
    book::Book,
    errors::Error,
    preprocess::{Preprocessor, PreprocessorContext},
};
use regex::Regex;

const CLASS_TWITTER: &str = "mdbook-embed-twitter";
const CLASS_YOUTUBE: &str = "mdbook-embed-youtube";

pub struct Embed;

impl Embed {
    pub fn new() -> Embed {
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
        let twitter_re = Regex::new(r"(https://twitter\.com.+)").unwrap();
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chap) = item {
                chap.content = embed_re.replace_all(&chap.content, |caps: &regex::Captures| {
                    let url = caps.name("url").unwrap().as_str().to_owned();
                    if let Some(cap) = youtube_re.captures_iter(&url).next() {
                        format!("<iframe class=\"{CLASS_YOUTUBE}\" width=\"560\" height=\"315\" src=\"https://www.youtube.com/embed/{}\"></iframe>", &cap[1])
                    } else if let Some(cap) = twitter_re.captures_iter(&url).next() {
                        format!(r#"<blockquote class="twitter-tweet {CLASS_TWITTER}">
                                       <a href="{}"></a>
                                   </blockquote>
                                   <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>"#, &cap[1])
                    } else {
                        format!("<a href=\"{url}\">{url}</a>")
                    }
                }).to_string();
            }
        });
        Ok(book)
    }
}
