# pquote

Programmer Quotes Library & Binary

[![Continuous Integration](https://github.com/Th3Whit3Wolf/pquote/workflows/Continuous%20Integration/badge.svg?branch=main&event=deployment_status)][CI]
[![Coverage Status](https://coveralls.io/repos/github/Th3Whit3Wolf/pquote/badge.svg)][Coverage]
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)][License]
[![Rust](https://img.shields.io/badge/rust-v1.36+-red.svg)][Rust]

[CI]: https://github.com/Th3Whit3Wolf/pquote/actions?query=workflow%3A%22Continuous+Integration%22
[Coverage]: https://coveralls.io/github/Th3Whit3Wolf/pquote
[Documentation]: https://th3whit3wolf.github.io/pquote/pquote/
[License]: https://opensource.org/licenses/Apache-2.0
[Rust]: https://www.rust-lang.org/

List of 323 Quotes from various programmer's

## Why

I really liked [Vim Startify's](https://github.com/mhinz/vim-startify) programmer quotes every time I used vim. I originally made this to get the same quotes for [Dashboard Nvim](https://github.com/hardcoreplayers/dashboard-nvim) but I ended up using it for MOTD on remote servers and some other random projects(I love programmer quotes) so I thought I might as well make it a library to make it easier to use.

## Why is the binary in the examples directory

The binary is an example because it means all of it's dependencies can be kept seperate from the dependencies of the library, (as far as I can tell there is no way to do this with the lib & main inside `src/` or with workspaces) so that if you use this library you're not pulling in 19 other crates you don't need.

## How to use

### Library

```toml
# Cargo.toml

[dependencies]
pquote = { git = "https://github.com/Th3Whit3Wolf/pquote" }
```

```rs
// main.rs or lib.rs

use pquote::QUOTES;
```

#### Example

Want to generate a random quote?

```rs
use rand::Rng;
use pquote::QUOTES;

fn main() {
    // Get Random Quote
    let x = rand::thread_rng().gen_range(0, QUOTES.len());
    // Print quote and author
    println!("Quote: {}", q.quote);
    println!("\t- {}", q.author);
}
```

Click [here](https://github.com/Th3Whit3Wolf/pquote/blob/master/examples/pq.rs) for a much more in-depth example.

### Binary

```sh
pquote 0.2.0
Programmer Quote Generator

USAGE:
    pquote <FLAGS>

FLAGS:
    -h,--help                Prints help information
    -V,--version             Prints version information
    -v,--verbose             Prints quote verbosely
    -A,--all                 Print all quotes
    -i,--id <id>             Choose quote by id
    -a,--author <author>     Choose quote by author
    -o,--origin <origin>     Choose quote by origin (azquotes,goodreads,journaldev,vimstartify,stormconsultancy)
```

#### Prequisites

* Rust - if not installed run `curl https://sh.rustup.rs -sSf | sh`

#### Install

```sh
git clone https://github.com/Th3Whit3Wolf/pquote.git
cd pquote
cargo install --example pq
```

#### Example Output

```txt
    The most fundamental problem in software development is complexity.
    There is only one basic way of dealing with complexity: divide and conquer

    - Bjarne Stroustrup
```

## Credits & Contribution

[Vim Startify](https://github.com/mhinz/vim-startify) with it's awesome quotes inspired to make this and I borrowed it's content.

[This Programming Quotes API](http://quotes.stormconsultancy.co.uk/api) also gave a good amount of quotes.
