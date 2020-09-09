use ansi_term::Colour::{Green, Red, Yellow};
use pquote::{Quote, QUOTES};
use rand::Rng;
use textwrap::Wrapper;

#[derive(Debug)]
enum AppArgs {
    Flags {
        help: bool,
        version: bool,
        verbose: bool,
        all: bool,
        id: Option<u32>,
        author: Option<String>,
        origin: Option<String>,
    },
}

fn submain() -> AppArgs {
    let args = pico_args::Arguments::from_env();

    match parse_flags(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!(
                "{} {}
{}
    pquote {}",
                Red.paint("Error:"),
                e,
                Yellow.paint("USAGE:"),
                Green.paint("--help")
            );
            std::process::exit(1);
        }
    }
}

// Determines how the flags are parsed
fn parse_flags(mut args: pico_args::Arguments) -> Result<AppArgs, pico_args::Error> {
    let app_args = AppArgs::Flags {
        help: args.contains(["-h", "--help"]),
        version: args.contains(["-V", "--version"]),
        verbose: args.contains(["-v", "--verbose"]),
        all: args.contains(["-A", "--all"]),
        id: args.opt_value_from_str(["-i", "--id"])?,
        author: args.opt_value_from_str(["-a", "--author"])?,
        origin: args.opt_value_from_str(["-o", "--origin"])?,
    };

    args.finish()?;
    Ok(app_args)
}

fn help_print() {
    println!(
        "{} {}
Programmer Quote Generator

{}
    pquote <FLAGS>

{}
    {},{}                Prints help information
    {},{}             Prints version information
    {},{}             Prints quote verbosely
    {},{}                 Print all quotes
    {},{}             Choose quote by id
    {},{}     Choose quote by author
    {},{}     Choose quote by origin (azquotes,goodreads,journaldev,vimstartify,stormconsultancy)
",
        Green.paint("pquote"),
        env!("CARGO_PKG_VERSION"),
        Yellow.paint("USAGE:"),
        Yellow.paint("FLAGS:"),
        Green.paint("-h"),
        Green.paint("--help"),
        Green.paint("-V"),
        Green.paint("--version"),
        Green.paint("-v"),
        Green.paint("--verbose"),
        Green.paint("-A"),
        Green.paint("--all"),
        Green.paint("-i"),
        Green.paint("--id <id>"),
        Green.paint("-a"),
        Green.paint("--author <author>"),
        Green.paint("-o"),
        Green.paint("--origin <origin>"),
    );
}

fn version_print() {
    println!("pquote {}", env!("CARGO_PKG_VERSION"))
}

fn main() {
    // Get Random Quote
    match submain() {
        AppArgs::Flags {
            help,
            version,
            verbose,
            all,
            id,
            author,
            origin,
        } => {
            let num = if let Some(x) = id {
                QUOTES.len() - x as usize
            } else {
                rand::thread_rng().gen_range(0, QUOTES.len())
            };
            if help {
                help_print();
            } else if version {
                version_print();
            } else if let Some(identity) = id {
                print_quote(identity as usize, verbose, all, None)
            } else if let Some(a) = author {
                let mut v: Vec<Quote> = Vec::with_capacity(1);
                for quotes in QUOTES.iter() {
                    if quotes.author == a {
                        v.push(*quotes)
                    }
                }
                match v.len() {
                    1 => print_quote(0, verbose, all, Some(v)),
                    l if l > 1 => {
                        print_quote(
                            rand::thread_rng().gen_range(0, v.len()),
                            verbose,
                            all,
                            Some(v),
                        );
                    }
                    _ => println!("Sorry no quotes found by {}", a),
                }
            } else if let Some(o) = origin {
                let mut v: Vec<Quote> = Vec::with_capacity(1);
                match o.as_str() {
                    "azquotes" | "azquote" | "AZQuote" | "AZQuotes" => {
                        for quote in QUOTES.iter() {
                            if quote.permalink.is_azquotes() {
                                v.push(*quote)
                            }
                        }
                    }
                    "goodreads" | "good reads" | "Good Reads" | "GoodReads" => {
                        for quote in QUOTES.iter() {
                            if quote.permalink.is_goodreads() {
                                v.push(*quote)
                            }
                        }
                    }
                    "journaldev" | "journal dev" | "Journaldev" | "Journal Dev" => {
                        for quote in QUOTES.iter() {
                            if quote.permalink.is_journaldev() {
                                v.push(*quote)
                            }
                        }
                    }
                    "vimstartify" | "vim startify" | "Vim Startify" | "Vimstartify" => {
                        for quote in QUOTES.iter() {
                            if quote.permalink.is_vimstartify() {
                                v.push(*quote)
                            }
                        }
                    }
                    "stormconsultancy" | "storm consultancy" | "Storm Consultancy"
                    | "Stormconsultancy" => {
                        for quote in QUOTES.iter() {
                            if quote.permalink.is_stormconsultancy() {
                                v.push(*quote)
                            }
                        }
                    }
                    _ => {}
                };

                match v.len() {
                    1 => print_quote(0, verbose, all, Some(v)),
                    l if l > 1 => {
                        print_quote(
                            rand::thread_rng().gen_range(0, v.len()),
                            verbose,
                            all,
                            Some(v),
                        );
                    }
                    _ => println!("Sorry no quotes found by {}", o),
                }
            } else {
                print_quote(num, verbose, all, None)
            }
        }
    }
}

fn print_quote(num: usize, verbose: bool, all: bool, list: Option<Vec<Quote>>) {
    let quotes = if let Some(x) = list {
        x
    } else {
        QUOTES.to_vec()
    };

    if all {
        for quote in quotes {
            if verbose {
                println!(
                    "ID: {}\nQuote: {}\nAuthor: {}\nLink: {}\n",
                    quote.id,
                    Wrapper::with_termwidth().fill(quote.quote),
                    quote.author,
                    quote.permalink,
                );
            } else {
                println!(
                    "{}\n\n\t- {}",
                    Wrapper::with_termwidth().fill(quote.quote),
                    quote.author
                );
            }
        }
    } else if verbose {
        println!(
            "ID: {}\nQuote: {}\nAuthor: {}\nLink: {}\n",
            quotes[num].id,
            Wrapper::with_termwidth().fill(quotes[num].quote),
            quotes[num].author,
            quotes[num].permalink,
        );
    } else {
        println!(
            "{}\n\n\t- {}",
            Wrapper::with_termwidth().fill(quotes[num].quote),
            quotes[num].author
        );
    }
}
