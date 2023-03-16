# mdbook-embed

[![Crates.io](https://img.shields.io/crates/v/mdbook-embed.svg)](https://crates.io/crates/mdbook-embed)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

A preprocessor that simplifies embedded URL.

## Example

It turns this:

```md
{{#embed https://www.youtube.com/watch?v=d66B35sT1gQ}}
```

into:

```md
<iframe width="560" height="315" src="https://www.youtube.com/embed/d66B35sT1gQ"></iframe>
```

## Installation

To install `mdbook-embed`, use cargo:

```
cargo install mdbook-embed
```

Then add the following to `book.toml`:

```
[preprocessor.embed]
```

## Support services

- [x] YouTube
- [ ] Twitter
- [ ] Instagram

