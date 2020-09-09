use crate::{Origin, QUOTES};

#[test]
fn all_quotes() {
    for q in QUOTES.iter() {
        println!("Quote: {}", q.id);
        println!("Origin: {}", q.permalink);
        println!("\t{}", q.quote);
        println!("\n\t\t{}\n", q.author);
    }
}

#[test]
fn grammar() -> std::result::Result<(), &'static str> {
    //static FALSE_POSITIVE: [u32;1] = [];
    let mut b_grammar = Vec::new();
    for q in QUOTES.iter() {
        if q.quote.chars().next().unwrap().is_ascii_lowercase()
            || q.author.chars().next().unwrap().is_ascii_lowercase()
        {
            b_grammar.push(q)
        }
    }
    if !b_grammar.is_empty() {
        println!("The following quotes do not start with captialized letter:");
        for i in b_grammar {
            println!("{}", i.id)
        }
        Err("Duplicates found")
    } else {
        Ok(())
    }
}

#[test]
fn duplicates_quote() -> std::result::Result<(), &'static str> {
    let mut vec = Vec::with_capacity(QUOTES.len());
    let mut dup: Vec<(usize, &str)> = Vec::new();
    for q in QUOTES.iter() {
        vec.push(q.quote)
    }
    vec.sort();
    for i in 0..vec.len() {
        if i > 0 && vec[i - 1] == vec[i] {
            dup.push((i, vec[i]))
        }
    }

    if !dup.is_empty() {
        println!("There are {} duplicates!", dup.len());
        for (x, y) in dup {
            println!("{}: {}", x, y)
        }
        Err("Duplicates found")
    } else {
        Ok(())
    }
}

#[test]
#[allow(clippy::needless_range_loop)]
fn wrong_id() -> std::result::Result<(), &'static str> {
    let mut wrong: Vec<(u32, usize)> = Vec::new();

    for i in 0..QUOTES.len() {
        if (QUOTES.len() - i) as u32 != QUOTES[i].id {
            wrong.push((QUOTES[i].id, QUOTES.len() - i))
        }
    }
    if !wrong.is_empty() {
        println!("There are {} wrong IDs!", wrong.len());
        for (x, y) in wrong {
            println!("ID{}: {}", x, y)
        }
        Err("Wrong IDs found")
    } else {
        Ok(())
    }
}

#[test]
fn authors() {
    let mut map: std::collections::HashMap<&'static str, u32> = std::collections::HashMap::new();
    for quote in QUOTES.iter() {
        *map.entry(quote.author).or_insert(0) += 1;
    }
    // Get length of largest key
    let j = map
        .keys()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();
    let map_clone = map.clone();
    let last = map_clone.keys().last().unwrap();
    println!("┌{}───────┬──────────┐", "─".repeat(j));
    println!("│ Author{}│ Quote(s) │", " ".repeat(j));
    println!("├{}───────┼──────────┤", "─".repeat(j));
    for (name, count) in map {
        match count {
            1..=9 => {
                if name.contains("“") {
                    println!(
                        "│ {}{} │    {}     │",
                        name,
                        " ".repeat(j - name.len() + 9),
                        count
                    )
                } else {
                    println!(
                        "│ {}{} │    {}     │",
                        name,
                        " ".repeat(j - name.len() + 5),
                        count
                    )
                }
            }
            10..=99 => println!(
                "│ {}{} │    {}    │",
                name,
                " ".repeat(j - name.len() + 5),
                count
            ),
            _ => println!(
                "│ {}{} │    {}  │",
                name,
                " ".repeat(j - name.len() + 5),
                count
            ),
        }
        if &name != last {
            println!("├{}───────┼──────────┤", "─".repeat(j));
        } else {
            println!("└{}───────┴──────────┘", "─".repeat(j));
        }
    }
}

#[test]
fn origins() -> std::result::Result<(), &'static str> {
    let mut v: Vec<(u32, Origin)> = Vec::new();
    for q in QUOTES.iter() {
        if q.permalink.is_azquotes()
            || q.permalink.is_azquotesauthor()
            || q.permalink.is_azquotesquote()
            || q.permalink.is_goodreads()
            || q.permalink.is_journaldev()
            || q.permalink.is_vimstartify()
            || q.permalink.is_stormconsultancy()
        {
            println!("Quote {}: has Origin: {}", q.id, q.permalink)
        } else {
            v.push((q.id, q.permalink))
        }
    }

    if !v.is_empty() {
        println!("There are {} unknown Origins!", v.len());
        for (x, y) in v {
            println!("Quote {}: has unkown Origin: {}", x, y)
        }
        Err("Unknown Origins found")
    } else {
        Ok(())
    }
}
