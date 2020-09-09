#[cfg(test)]
mod test;

/// All quotes must have the following
///
/// * Author
/// * Sequential id
/// * Quote (duh)
/// * Permalink where the quote was found
#[derive(Debug, Copy, Clone)]
pub struct Quote {
    pub author: &'static str,
    pub id: u32,
    pub quote: &'static str,
    pub permalink: Origin,
}

/// Origin of a quote
#[derive(Debug, Copy, Clone)]
pub enum Origin {
    AZQuotesQuote(u32),
    AZQuotesAuthor(&'static str),
    GoodReads,
    JournalDev,
    VimStartify,
    StormConsultancy(u8),
}

/// Check if quote comes from [AZ Quotes](https://www.azquotes.com)
impl Origin {
    #[must_use]
    pub fn is_azquotes(&self) -> bool {
        matches!(*self, Origin::AZQuotesQuote(_) | Origin::AZQuotesAuthor(_))
    }
}

/// Check if quote comes from [AZ Quotes - Quote Category](https://www.azquotes.com/quote/755_276)
impl Origin {
    #[must_use]
    pub fn is_azquotesquote(&self) -> bool {
        matches!(*self, Origin::AZQuotesQuote(_))
    }
}

/// Check if quote comes from [AZ Quotes - Author Category](https://www.azquotes.com/quotes/authors.html)
impl Origin {
    #[must_use]
    pub fn is_azquotesauthor(&self) -> bool {
        matches!(*self, Origin::AZQuotesAuthor(_))
    }
}

/// Check if quote comes from [goodreads](https://www.goodreads.com/quotes/tag/programming)
impl Origin {
    #[must_use]
    pub fn is_goodreads(&self) -> bool {
        matches!(*self, Origin::GoodReads)
    }
}

/// Check if quote comes from [Journal Dev](https://www.journaldev.com/240/my-25-favorite-programming-quotes-that-are-funny-too)
impl Origin {
    #[must_use]
    pub fn is_journaldev(&self) -> bool {
        matches!(*self, Origin::JournalDev)
    }
}

/// Check if quote comes from [Vim Startify](https://github.com/mhinz/vim-startify)
impl Origin {
    #[must_use]
    pub fn is_vimstartify(&self) -> bool {
        matches!(*self, Origin::VimStartify)
    }
}

/// Check if quote comes from [Storm Consultancy](http://quotes.stormconsultancy.co.uk/)
impl Origin {
    #[must_use]
    pub fn is_stormconsultancy(&self) -> bool {
        matches!(*self, Origin::StormConsultancy(_))
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Origin::AZQuotesQuote(num) => {
                write!(f, "https://www.azquotes.com/quote/{}", num)
            },
            Origin::AZQuotesAuthor(str) => {
                write!(f, "https://www.azquotes.com/author/{}", str)
            },
            Origin::GoodReads => {
                write!(f, "https://www.goodreads.com/quotes/tag/programming")
            },
            Origin::JournalDev => {
                write!(f, "https://www.journaldev.com/240/my-25-favorite-programming-quotes-that-are-funny-too")
            },
            Origin::VimStartify => {
                write!(f, "https://github.com/mhinz/vim-startify/blob/master/autoload/startify/fortune.vim")
            },
            Origin::StormConsultancy(num) => {
                write!(f, "http://quotes.stormconsultancy.co.uk/quotes/{}", num)
            }
        }
    }
}

/// Array of (323) Programming related quotes
pub static QUOTES: [Quote; 323] = [
    Quote {
        author: "Linus Torvalds",
        id: 323,
        quote: "I think Leopard is a much better system [[than Windows Vista]] but OS X in some ways is actually worse than Windows to program for. Their file system is complete and utter crap, which is scary.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 322,
        quote: "Hmmm, completely a-religious - atheist. I find that people seem to think religion brings morals and appreciation of nature. I actually think it detracts from both.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 321,
        quote: "On the internet nobody can hear you being subtle.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 320,
        quote: "Only religious fanatics and totalitarian states equate morality with legality.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 319,
        quote: "Security people are often the black-and-white kind of people that I can't stand. I think the OpenBSD crowd is a bunch of masturbating monkeys, in that they make such a big deal about concentrating on security to the point where they pretty much admit that nothing else matters to them.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 318,
        quote: "The complaints I've had is that GitHub as a development platform - making commits, pull requests, keeping track of issues etc - doesn't work very well at all. It's not even close, not for something like the kernel. It's much too limited.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 317,
        quote: "There are lots of Linux users who don't care how the kernel works but only want to use it is not only a tribute to how good Linux is, but it also brings up issues that I would never have thought of otherwise.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 316,
        quote: "I get the biggest enjoyment from the random and unexpected places. Linux on cellphones or refrigerators, just because it's so not what I envisioned it. Or on supercomputers.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 315,
        quote: "In many cases the user interface to a program is the most important part for a commercial company: whether the programs works correctly or not seems to be secondary.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 314,
        quote: "It's what I call \"mental masturbation\", when you engage is some pointless intellectual exercise that has no possible meaning.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 313,
        quote: "Most of the good programmers do programming not because they expect to get paid or get adulation by the public, but because it is fun to program.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 312,
        quote: "Intelligence is the ability to avoid doing work, yet getting the work done.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 311,
        quote: "If you think penguins are fat and waddle, you have never been attacked by one running at you in excess of 100 miles per hour.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 310,
        quote: "Software is like sex: It's better when it's free.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 309,
        quote: "In short: just say NO TO DRUGS, and maybe you won't end up like the Hurd people.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 308,
        quote: "Programmers are in the enviable position of not only getting to do what they want to, but because the end result is so important they get paid to do it. There are other professions like that, but not that many.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 307,
        quote: "To be a nemesis, you have to actively try to destroy something, don't you? Really, I'm not out to destroy Microsoft. That will just be a completely unintentional side effect.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 306,
        quote: "UNIX has a philosophy, it has 25 years of history behind it, and most importantly, it has a clean core. It strives for something - some kind of beauty. And that's really what struck me as a programmer. Operating systems that normal home users are used to, such as DOS and Windows, didn't have any way of life. Nobody tried to design Windows - it just grew in random directions without any kind of thought behind it. [...] I don't think Microsoft is evil in itself; I just think that they make really crappy operating systems.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 305,
        quote: "When you say \"I wrote a program that crashed Windows,\" people just stare at you blankly and say \"Hey, I got those with the system, for free.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 304,
        quote: "I started Linux as a desktop operating system. And it's the only area where Linux hasn't completely taken over. That just annoys the hell out of me.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 303,
        quote: "Don't hurry your code. Make sure it works well and is well designed. don't worry about timing.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 302,
        quote: "C++ is a horrible language. It's made more horrible by the fact that a lot of substandard programmers use it, to the point where it's much much easier to generate total and utter crap with it.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 301,
        quote: "Those that can, do. Those that can't, complain.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 300,
        quote: "Backups are for wimps. Real men upload their data to an FTP site and have everyone else mirror it.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 299,
        quote: "If Microsoft ever does applications for Linux it means I'vewon.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 298,
        quote: "Modern PCs are horrible. ACPI is a complete design disaster in every way. But we're kind of stuck with it. If any Intel people are listening to this and you had anything to do with ACPI, shoot yourself now, before you reproduce.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 297,
        quote: "Microsoft isn't evil, they just make really crappy operating systems.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 296,
        quote: "Bad programmers worry about the code. Good programmers worry about data structures and their relationships.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 295,
        quote: "Theory and practice sometimes clash. And when that happens, theory loses. Every single time.",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Linus Torvalds",
        id: 294,
        quote: "A computer is like air conditioning - it becomes useless when you open Windows",
        permalink: Origin::AZQuotesQuote(755_276)
    },
    Quote {
        author: "Richard Stallman",
        id: 293,
        quote: "Proprietary software tends to have malicious features. The point is with a proprietary program, when the users dont have the source code, we can never tell. So you must consider every proprietary program as potential malware.",
        permalink: Origin::AZQuotesAuthor("13994-Richard_Stallman"),
    },
    Quote {
        author: "Richard Stallman",
        id: 292,
        quote: "Android is very different from the GNU/Linux operating system because it contains very little of GNU. Indeed, just about the only component in common between Android and GNU/Linux is Linux, the kernel.",
        permalink: Origin::AZQuotesAuthor("13994-Richard_Stallman"),
    },
    Quote {
        author: "Richard Stallman",
        id: 291,
        quote: "Programming is not a science. Programming is a craft.",
        permalink: Origin::AZQuotesAuthor("13994-Richard_Stallman"),
    },
    Quote {
        author: "Richard Stallman",
        id: 290,
        quote: "Giving the Linus Torvalds Award to the Free Software Foundation is a bit like giving the Han Solo Award to the Rebel Alliance.",
        permalink: Origin::AZQuotesAuthor("13994-Richard_Stallman")
    },
    Quote {
        author: "Richard Stallman",
        id: 289,
        quote: "Sharing knowledge is the most fundamental act of friendship. Because it is a way you can give something without loosing something.",
        permalink: Origin::AZQuotesQuote(1_394_515)
    },
    Quote {
        author: "Frederick P. Brooks Jr., The Mythical Man-Month: Essays on Software Engineering",
        id: 288,
        quote: "Einstein repeatedly argued that there must be simplified explanations of nature, because God is not capricious or arbitrary. No such faith comforts the software engineer.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Gerry Geek, Ice Breakers for Project Managers: Jokes, Quotes, and Brainteasers",
        id: 287,
        quote: "A code is like love, it has created with clear intentions at the beginning, but it can get complicated.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Herbert Schildt, C++: The Complete Reference",
        id: 286,
        quote: "C gives the programmer what the programmer wants; few restrictions, few complaints... C++ maintains the original spirit of C, that the programmer not the language is in charge.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Richard Stallman",
        id: 285,
        quote: "With software there are only two possibilites: either the users control the programme or the programme controls the users. If the programme controls the users, and the developer controls the programme, then the programme is an instrument of unjust power",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Dr. Hazem Ali",
        id: 284,
        quote: "The happiest moment i felt; is that moment when i realized my ability to create.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Chris Pine, Learn to Program",
        id: 283,
        quote: "Programming isn't about what you know; it's about what you can figure out.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Marijn Haverbeke, Eloquent JavaScript: A Modern Introduction",
        id: 282,
        quote: "The main thing I want to show in this chapter is that there is no magic involved in building your own language. I've often felt that some human inventions were so immensely clever and complicated that I'd never be able to understand them. But with a little reading and tinkering, such things often turn out to be quite mundane.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Neal Ford, Functional Thinking",
        id: 281,
        quote: "The problem with a completely new programming paradigm isn't learning a new language. After all, everyone reading this has learned numerous computer languages language syntax is merely details. The tricky part is learning to think in a different way.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Andrew Edward Lucier, Awakenigma Allegory Anomalous",
        id: 280,
        quote: "Reality really relies on authoritatively regulating. Your absolute attention",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Mokokoma Mokhonoana",
        id: 279,
        quote: "Most improved things can be improved.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Anthony T. Hincks",
        id: 278,
        quote: "Sometimes, I dream of becoming real, but I don't know if that is real, or just part of my programming.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Joe Armstrong",
        id: 277,
        quote: "The really good programmers spend a lot of time programming. I haven't seen very good programmers who don't spend a lot of time programming. If I don't program for two or three days, I need to do it. And you get better at it-you get quicker at it. The side effect of writing all this other stuff is that when you get to doing ordinary problems, you can do them very quickly.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Bartosz Milewski, Category Theory for Programmers",
        id: 276,
        quote: "The usual goal in the typing monkeys thought experiment is the production of the complete works of Shakespeare. Having a spell checker and a grammar checker in the loop would drastically increase the odds. The analog of a type checker would go even further by making sure that, once Romeo is declared a human being, he doesn't sprout leaves or trap photons in his powerful gravitational field.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Robert C. Martin, Agile Principles, Patterns, and Practices in C#",
        id: 275,
        quote: "Abstraction is the elimination of the irrelevant and the amplification of the essential.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Halgurd Hussein",
        id: 274,
        quote: "I am committed to push my branch to the master",
        permalink: Origin::GoodReads
    },
    Quote {
        author:"John Drury Clark, Ignition!: An informal history of liquid rocket propellants",
        id: 273,
        quote: "And there is one disconcerting thing about working with a computer - it's likely to talk back to you. You make some tiny mistake in your FORTRAN language - putting a letter in the wrong column, say, or omitting a comma - and the 360 comes to a screeching halt and prints out rude remarks, like \"ILLEGAL FORMAT,\" or \"UNKNOWN PROBLEM,\" or, if the man who wrote the program was really feeling nasty that morning, \"WHAT'S THE MATTER STUPID? CAN'T YOU READ?\" Everyone who uses a computer frequently has had, from time to time, a mad desire to attack the precocious abacus with an axe.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Steve McConnell, Code Complete",
        id: 272,
        quote: "The big optimizations come from refining the high-level design, not the individual routines.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Joseph Rain, The Unfinished Book About Who We Are",
        id: 271,
        quote: "Along every step of our journey through life, our mind is being programmed. If we are not programming it ourselves, someone else is doing it to us.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Seymour Cray",
        id: 270,
        quote: "The trouble with programmers is that you can never tell what a programmer is doing until it's too late.",
        permalink:Origin::JournalDev
    },
    Quote {
        author: "J. Osterhout",
        id: 269,
        quote: "The best performance improvement is the transition from the nonworking state to the working state.",
        permalink:Origin::JournalDev
    },
    Quote {
        author: "Gordon Bell",
        id: 268,
        quote: "The cheapest, fastest, and most reliable components are those that aren't there.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Keith Braithwaite",
        id: 267,
        quote: "Itss a curious thing about our industry: not only do we not learn from our mistakes, but we also don't learn from our successes",
        permalink:Origin::JournalDev
    },
    Quote {
        author: "Unknown",
        id: 266,
        quote: "Software undergoes beta testing shortly before it\u{2019}s released. Beta is Latin for \u{201c}still doesn\u{2019}t work.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Jeff Sickel",
        id: 265,
        quote: "Deleted code is debugged code.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Jessica Gaston",
        id: 264,
        quote: "One man's crappy software is another man's full-time job.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Unknown",
        id: 263,
        quote: "It's not a bug - it's an undocumented feature.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Unknown",
        id: 262,
        quote: "Ready, fire, aim: the fast approach to software development. Ready, aim, aim, aim, aim: the slow approach to software development",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Alan J. Perlis",
        id: 261,
        quote: "There are two ways to write error-free programs; only the third one works.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Gerald Weinberg",
        id: 260,
        quote: "If builders built buildings the way programmers wrote programs, then the first woodpecker that came along would destroy civilization.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Oktal",
        id: 259,
        quote: "I think Microsoft named .Net so it wouldn\u{2019}t show up in a Unix directory listing.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Unknown",
        id: 258,
        quote: "The best method for accelerating a computer is the one that boosts it by 9.8 m/s2.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Ralph Johnson",
        id: 257,
        quote: "Before software can be reusable it first has to be usable",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Louis Srygley",
        id: 256,
        quote: "Without requirements or design, programming is the art of adding bugs to an empty text file.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Unknown",
        id: 255,
        quote: "The best thing about a boolean is even if you are wrong, you are only off by a bit.",
        permalink: Origin::JournalDev
    },
    Quote {
        author: "Steve Jobs",
        id: 254,
        quote: "Here's to the crazy ones, the misfits, the rebels, the troublemakers, the round pegs in the square holes... the ones who see things differently -- they're not fond of rules... You can quote them, disagree with them, glorify or vilify them, but the only thing you can't do is ignore them because they change things... they push the human race forward, and while some may see them as the crazy ones, we see genius, because the ones who are crazy enough to think that they can change the world, are the ones who do.",
        permalink: Origin::AZQuotesQuote(1_367_134)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 253,
        quote: "And no, I'm not a walking C++ dictionary. I do not keep every technical detail in my head at all times. If I did that, I would be a much poorer programmer. I do keep the main points straight in my head most of the time, and I do know where to find the details when I need them.",
        permalink: Origin::AZQuotesQuote(1_367_134)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 252,
        quote: "Certainly not every good program is object-oriented, and not every object-oriented program is good.",
        permalink: Origin::AZQuotesQuote(819_493)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 251,
        quote: "Design and programming are human activities; forget that and all is lost.",
        permalink: Origin::AZQuotesQuote(673_772)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 250,
        quote: "People who passionately want to believe that the world is basically simple react to this with a fury that goes beyond what I consider appropriate for discussing a programming language.",
        permalink: Origin::AZQuotesQuote(286_527)
    },
    Quote {
        author: "Bjarne Stroustrup (2000). \u{201c}The C++ Programming Language\u{201d}, Addison-Wesley Professional",
        id: 249,
        quote: "To many managers, getting rid of the arrogant, undisciplined, over-paid, technology-obsessed, improperly-dressed etc. programmers would appear to be a significant added benefit",
        permalink: Origin::AZQuotesQuote(1_338_293)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 248,
        quote: "\"How to test?\" is a question that cannot be answered in general. \"When to test?\" however, does have a general answer: as early and as often as possible.",
        permalink: Origin::AZQuotesQuote(1_103_626)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 247,
        quote: "Our civilization depends critically on software, and we have a dangerously low degree of professionalism in the computer fields",
        permalink: Origin::AZQuotesQuote(819_497)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 246,
        quote: "There are more useful systems developed in languages deemed awful than in languages praised for being beautiful - many more.",
        permalink: Origin::AZQuotesQuote(1_103_627)
    },
    Quote {
        author: "\"The C++ Programming Language\". Book by Bjarne Stroustrup, October 1985.",
        id: 245,
        quote: "An organisation that treats its programmers as morons will soon have programmers that are willing and able to act like morons only.",
        permalink: Origin::AZQuotesQuote(673_771)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 244,
        quote: "There is no one \"root of all evil\" in software development. Design is hard in many ways. People tend to underestimate the intellectual and practical difficulties involved in building a significant system involving software. It is not and will not be reduced to a simple mechanical \"assembly line\" process. Creativity, engineering principles, and evolutionary change are needed to create a satisfactory large system.",
        permalink: Origin::AZQuotesQuote(1_521_706)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 243,
        quote: "If you think it's simple, then you have misunderstood the problem.",
        permalink: Origin::AZQuotesQuote(765_017)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 242,
        quote: "Anybody who comes to you and says he has a perfect language is either naive or a salesman.",
        permalink: Origin::AZQuotesQuote(819_484)
    },
    Quote {
        author: "Bjarne Stroustrup (1991). \u{201c}C Plus Plus Programming Language\u{201d}, Addison-Wesley / Helix Books ",
        id: 241,
        quote: "Destructors for virtual base classes are executed in the reverse order of their appearance in a depth-first left-to-right traversal of the directed acyclic graph of base classes.",
        permalink: Origin::AZQuotesQuote(1_426_519)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 240,
        quote: "The first law of computer science: Every problem is solved by yet another indirection.",
        permalink: Origin::AZQuotesQuote(1_339_165)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 239,
        quote: "It's easy to win forgiveness for being wrong; being right is what gets you into real trouble.",
        permalink: Origin::AZQuotesQuote(819_481)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 238,
        quote: "C++ is my favorite garbage collected language because it generates so little garbage",
        permalink: Origin::AZQuotesQuote(819_485)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 237,
        quote: "Java isn't platform independent; it is a platform",
        permalink: Origin::AZQuotesQuote(1_122_900)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 236,
        quote: "The most important single aspect of software development is to be clear about what you are trying to build.",
        permalink: Origin::AZQuotesQuote(700_190)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 235,
        quote: "A program that has not been tested does not work.",
        permalink: Origin::AZQuotesQuote(819_483)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 234,
        quote: "When done well, software is invisible.",
        permalink: Origin::AZQuotesQuote(875_481)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 233,
        quote: "Tom [Cargil]s suggestion with a further idea: Propsers of new [C++] features should be required to donate a kidney. That would - Jim [Waldo] pointed out - make people think hard before proposing, and even people without any sense would propose at most two extensions.",
        permalink: Origin::AZQuotesQuote(819_496)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 232,
        quote: "The most fundamental problem in software development is complexity. There is only one basic way of dealing with complexity: divide and conquer",
        permalink: Origin::AZQuotesQuote(819_498)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 231,
        quote: "C++ is designed to allow you to express ideas, but if you don't have ideas or don't have any clue about how to express them, C++ doesn't offer much help.",
        permalink: Origin::AZQuotesQuote(819_482)
    },
    Quote {
        author: "Robert C. Martin, Clean Architecture",
        id: 230,
        quote: "Any organisation that designs a system will produce a design whose structure is a copy of the organisation's communication structure",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Abhijit Naskar",
        id: 229,
        quote: "Artificial intelligence is nowhere near attaining actual sentience or awareness. And without awareness it\u{2019}s simply a mechanical device, which may pretend to show emotions and sentience, if it is programmed to do so, and thus it may be able to fool the humans as being alive, but in its own internal circuitry, it\u{2019}d simply be following its preprogrammed tasks through the flowchart of an algorithm.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Donald Knuth",
        id: 228,
        quote: "Everyday life is like programming, I guess. If you love something you can put beauty into it.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Robert C. Martin, Clean Architecture",
        id: 227,
        quote: "The only way to go fast, is to go well.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Akshat Paul, React Native for iOS Development ",
        id: 226,
        quote: "User interface is the process of shifting from chaotic complexity to elegant simplicity.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Scott Meyers, Effective C++: 55 Specific Ways to Improve Your Programs and Designs ",
        id: 225,
        quote: "That doesn't upset too many people, but the fact that accessibility restrictions don't enter into the picture has caused more than one otherwise pacifistic soul to contemplate distinctly unpacifistic actions.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Alan J. Perlis",
        id: 224,
        quote: "What's in your hands I think and hope is intelligence: the ability to see the machine as more than when you were \u{fb01}rst led up to it that you can make it more.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Robert C. Martin, Clean Architecture",
        id: 223,
        quote: "I'm a programmer. I like programming. And the best way I'vefound to have a positive impact on code is to write it.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Vernor Vinge, A Deepness in the Sky",
        id: 222,
        quote: "Programming went back to the beginning of time. It was a little like the midden out back of his father's castle.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "James Alan Gardner, Ascending",
        id: 221,
        quote: "What kind of programmer is so divorced from reality that she thinks she'll get complex software right the first time?",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Alan J Perlis",
        id: 220,
        quote: "Is it possible that software is not like anything else, that it is meant to be discarded: that the whole point is to always see it as a soap bubble",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Steven S. Skiena, The Algorithm Design Manual",
        id: 219,
        quote: "The issue of finding the best possible answer or achieving maximum efficiency usually arises in industry only after serious performance or legal troubles.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Andrew Hunt, The Pragmatic Programmer: From Journeyman to Master",
        id: 218,
        quote: "Don\"t gloss over a routine or piece of code involved in the bug because you \"know\" it works. Prove it. Prove it in this context, with this data, with these boundary conditions.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Robert C. Martin",
        id: 217,
        quote: "Remember that code is really the language in which we ultimately express the requirements. We may create languages that are closer to the requirements. We may create tools that help us parse and assemble those requirements into formal structures. But we will never eliminate necessary precision\u{2014}so there will always be code.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Douglas Crockford, JavaScript: The Good Parts",
        id: 216,
        quote: "We see a lot of feature-driven product design in which the cost of features is not properly accounted. Features can have a negative value to customers because they make the products more difficult to understand and use. We are finding that people like products that just work. It turns out that designs that just work are much harder to produce that designs that assemble long lists of features.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Charles Petzold, Code",
        id: 215,
        quote: "Code is not like other how-computers-work books. It doesn't have big color illustrations of disk drives with arrows showing how the data sweeps into the computer. Code has no drawings of trains carrying a cargo of zeros and ones. Metaphors and similes are wonderful literary devices but they do nothing but obscure the beauty of technology.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Edsger W. Dijkstra",
        id: 214,
        quote: "Progress is possible only if we train ourselves to think about programs without thinking of them as pieces of executable code.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Alan J. Perlis ",
        id: 213,
        quote: "Programmers are not to be measured by their ingenuity and their logic but by the completeness of their case analysis.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Max Kanat-Alexander, Code Simplicity: The Fundamentals of Software",
        id: 212,
        quote: "Some of the best programming is done on paper, really. Putting it into the computer is just a minor detail.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Douglas Rushkoff, Program or Be Programmed: Ten Commands for a Digital Age",
        id: 211,
        quote: "We are looking at a society increasingly dependent on machines, yet decreasingly capable of making or even using them effectively.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Edmond Mbiaka ",
        id: 210,
        quote: "Take positive care of your mind, and it would surely take positive care of your life.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Suzy Kassem, Rise Up and Salute the Sun: The Writings of Suzy Kassem",
        id: 209,
        quote: "A conscious human is driven by their conscience, not popular opinion.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Alan J. Perlis",
        id: 208,
        quote: "I think that it\u{2019}s extraordinarily important that we in computer science keep fun in computing. When it started out it was an awful lot of fun. Of course the paying customers got shafted every now and then and after a while we began to take their complaints seriously. We began to feel as if we really were responsible for the successful error-free perfect use of these machines. I don\u{2019}t think we are. I think we\u{2019}re responsible for stretching them setting them off in new directions and keeping fun in the house. I hope the \u{fb01}eld of computer science never loses its sense of fun. Above all I hope we don\u{2019}t become missionaries. Don\u{2019}t feel as if you\u{2019}re Bible sales-men. The world has too many of those already. What you know about computing other people will learn. Don\u{2019}t feel as if the key to successful computing is only in your hands. What\u{2019}s in your hands I think and hope is intelligence: the ability to see the machine as more than when you were \u{fb01}rst led up to it that you can make it more.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Michael Crichton, Prey",
        id: 207,
        quote: "At forty, I was too old to work as a programmer myself anymore; writing code is a young person\u{2019}s job.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Marvin Minsky",
        id: 206,
        quote: "A computer is like a violin. You can imagine a novice trying \u{fb01}rst a phonograph and then a violin. The latter, he says, sounds terrible. That is the argument we have heard from our humanists and most of our computer scientists. Computer programs are good, they say, for particular purposes, but they aren\u{2019}t \u{fb02}exible. Neither is a violin, or a typewriter, until you learn how to use it.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Larry Wall ",
        id: 205,
        quote: "When they first built the University of California at Irvine they just put the buildings in. They did not put any sidewalks, they just planted grass. The next year, they came back and put the sidewalks where the trails were in the grass. Perl is just that kind of language. It is not designed from first principles. Perl is those sidewalks in the grass.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "C.A.R. Hoare",
        id: 204,
        quote: "The most important property of a program is whether it accomplishes the intention of its user.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Joseph Weizenbaum",
        id: 203,
        quote: "The computer programmer is a creator of universes for which he alone is the lawgiver. No playwright, no stage director, no emperor, however powerful, has ever exercised such absolute authority to arrange a stage or field of battle and to command such unswervingly dutiful actors or troops.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Alan Kay",
        id: 202,
        quote: "The most disastrous thing that you can ever learn is your first programming language.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Alan J. Perlis",
        id: 201,
        quote: "A language that doesn't affect the way you think about programming is not worth knowing.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Steve Jobs ",
        id: 200,
        quote: "You've baked a really lovely cake, but then you've used dog shit for frosting.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Robert C. Martin, Clean Code: A Handbook of Agile Software Craftsmanship ",
        id: 199,
        quote: "Truth can only be found in one place: the code.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Rasheed Ogunlaru",
        id: 198,
        quote: "How you look at it is pretty much how you'll see it",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Martin Fowler ",
        id: 197,
        quote: "Any fool can write code that a computer can understand. Good programmers write code that humans can understand.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Muhammad Waseem",
        id: 196,
        quote: "Give a man a program, frustrate him for a day.\n Teach a man to program, frustrate him for a lifetime.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Kent Beck",
        id: 195,
        quote: "I'm not a great programmer; I'm just a good programmer with great habits",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Larry Niven",
        id: 194,
        quote: "That's the thing about people who think they hate computers. What they really hate is lousy programmers.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Donald E. Knuth, Selected Papers on Computer Science ",
        id: 193,
        quote: "The best programs are written so that computing machines can perform them quickly and so that human beings can understand them clearly. A programmer is ideally an essayist who works with traditional aesthetic and literary forms as well as mathematical concepts, to communicate the way that an algorithm works and to convince a reader that the results will be correct.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Richard P. Feynman, Surely You're Joking, Mr. Feynman!: Adventures of a Curious Character",
        id: 192,
        quote: "Well, Mr. Frankel, who started this program, began to suffer from the computer disease that anybody who works with computers now knows about. It's a very serious disease and it interferes completely with the work. The trouble with computers is you *play* with them. They are so wonderful. You have these switches - if it's an even number you do this, if it's an odd number you do that - and pretty soon you can do more and more elaborate things if you are clever enough, on one machine. After a while the whole system broke down. Frankel wasn't paying any attention; he wasn't supervising anybody. The system was going very, very slowly - while he was sitting in a room figuring out how to make one tabulator automatically print arc-tangent X, and then it would start and it would print columns and then bitsi, bitsi, bitsi, and calculate the arc-tangent automatically by integrating as it went along and make a whole table in one operation. Absolutely useless. We *had* tables of arc-tangents. But if you've ever worked with computers, you understand the disease - the *delight* in being able to see how much you can do. But he got the disease for the first time, the poor fellow who invented the thing.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Harold Abelson, Structure and Interpretation of Computer Programs",
        id: 191,
        quote: "Programs must be written for people to read, and only incidentally for machines to execute.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Why The Lucky Stiff",
        id: 190,
        quote: "When you don't create things, you become defined by your tastes rather than ability. your tastes only narrow & exclude people. so create.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Linus Torvalds",
        id: 189,
        quote: "Talk is cheap. Show me the code.",
        permalink: Origin::GoodReads
    },
    Quote {
        author: "Frederick P. Brooks",
        id: 188,
        quote: "What one programmer can do in one month, two programmers can do in two months.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Aaron Swartz",
        id: 187,
        quote: "Be curious. Read widely. Try new things. I think a lot of what people call intelligence boils down to curiosity.",
        permalink:Origin::VimStartify
    },
    Quote {
        author: "Edward V. Berard",
        id: 186,
        quote: "Walking on water and developing software from a specification are easy if both are frozen.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Daniel J. Bernstein",
        id: 185,
        quote: "The average user doesn't give a damn what happens, as long as (1) it works and (2) it's fast.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Patrick McKenzie",
        id: 184,
        quote: "Every great developer you know got there by solving problems they were unqualified to solve until they actually did it.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Jon Acuff",
        id: 183,
        quote: "Developing tolerance for imperfection is the key factor in turning chronic starters into consistent finishers.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Stephen Hawking",
        id: 182,
        quote: "To understand recursion, one must first understand recursion.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Bill Gates, 1981",
        id: 181,
        quote: "640K ought to be enough for anybody.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Edsger W. Dijkstra",
        id: 180,
        quote: "If debugging is the process of removing bugs, then programming must be the process of putting them in.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Tom Cargill",
        id: 179,
        quote: "The first 90% of the code accounts for the first 90% of the development time. The remaining 10% of the code accounts for the other 90% of the development time.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 178,
        quote: "Question: How does a large software project get to be one year late? Answer: One day at a time!",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Paul Graham, Hackers & Painters: Big Ideas from the Computer Age",
        id: 177,
        quote: "The object-oriented model makes it easy to build up programs by accretion. What this often means, in practice, is that it provides a structured way to write spaghetti code.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Mark Twain",
        id: 176,
        quote: "They did not know it was impossible, so they did it!",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Donald Knuth",
        id: 175,
        quote: "Software is hard.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Terje Mathisen",
        id: 174,
        quote: "All programming is an exercise in caching.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Albert Einstein",
        id: 173,
        quote: "If you can't explain something to a six-year-old, you really don't understand it yourself.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 172,
        quote: "Nothing is more permanent than a temporary solution.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Larry Wall",
        id: 171,
        quote: "Easy things should be easy and hard things should be possible.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan J. Perlis",
        id: 170,
        quote: "Functions delay binding; data structures induce binding. Moral: Structure data late in the programming process.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "David Wheeler",
        id: 169,
        quote: "All problems in computer science can be solved with another level of indirection.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan J. Perlis",
        id: 168,
        quote: "A LISP programmer knows the value of everything, but the cost of nothing.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Mosher's Law of Software Engineering",
        id: 167,
        quote: "Don't worry if it doesn't work right. If everything did, you'd be out of a job",

        permalink: Origin::VimStartify
    },
    Quote {
        author: "Bill Gates",
        id: 166,
        quote: "Measuring programming progress by lines of code is like measuring aircraft building progress by weight.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "John Johnson",
        id: 165,
        quote: "First, solve the problem. Then, write the code.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Kent Beck",
        id: 164,
        quote: "Optimism is an occupational hazard of programming; feedback is the treatment.",
        permalink:  Origin::VimStartify
    },
    Quote {
        author: "Eric S. Raymond",
        id: 163,
        quote: "Computer science education cannot make anybody an expert programmer any more than studying brushes and pigment can make somebody an expert painter.",
        permalink:  Origin::VimStartify
    },
    Quote {
        author: "Larry Wall",
        id: 162,
        quote: "Most of you are familiar with the virtues of a programmer. There are three, of course: laziness, impatience, and hubris.",
        permalink:  Origin::VimStartify
    },
    Quote {
        author: "Edward Tufte",
        id: 161,
        quote: "There are only two industries that refer to their customers as \"users\".",
        permalink:
            Origin::VimStartify
    },
    Quote {
        author: "Grady Booch",
        id: 160,
        quote: "The function of good software is to make the complex appear to be simple.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Ray Ozzie",
        id: 159,
        quote: "Complexity kills. It sucks the life out of developers, it makes products difficult to plan, build and test, it introduces security challenges, and it causes end-user and administrator frustration.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Brian Kernighan",
        id: 158,
        quote: "Controlling complexity is the essence of computer programming.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Linus Torvalds",
        id: 157,
        quote: "The bulk of all patents are crap. Spending time reading them is stupid. It's up to the patent owner to do so, and to enforce them.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Dennie van Tassel",
        id: 156,
        quote: "I've finally learned what \"upward compatible\" means. It means we get to keep all our old mistakes.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Kay",
        id: 155,
        quote: "Most software today is very much like an Egyptian pyramid with millions of bricks piled on top of each other, with no structural integrity, but just done by brute force and thousands of slaves.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "C3PO",
        id: 154,
        quote: "The city's central computer told you? R2D2, you know better than to trust a strange computer!",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Marvin Minsky",
        id: 153,
        quote: "It's ridiculous to live 100 years and only be able to remember 30 million bytes. You know, less than a compact disc. The human condition is really becoming more obsolete every minute.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Edsger W. Dijkstra",
        id: 152,
        quote: "The question of whether computers can think is like the question of whether submarines can swim.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Pablo Picasso",
        id: 151,
        quote: "Computers are useless. They can only give you answers.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Peter Deutsch",
        id: 150,
        quote: "To iterate is human, to recurse divine.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 149,
        quote: "Weeks of programming can save you hours of planning.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 148,
        quote: "Why do we never have time to do it right, but always have time to do it over?",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 147,
        quote: "No matter how far down the wrong road you have gone, turn back now.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 146,
        quote: "Think twice, code once.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 145,
        quote: "Sign your work. Craftsmen of an earlier age were proud to sign their work. You should be, too.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 144,
        quote: "Find bugs once. Once a human tester finds a bug, it should be the last time a human tester finds that bug. Automatic tests should check for it from then on.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 143,
        quote: "Use saboteurs to test your testing. Introduce bugs on purpose in a separate copy of the source to verify that testing will catch them.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 142,
        quote: "Test early. Test often. Test automatically. Tests that run with every build are much more effective than test plans that sit on a shelf.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 141,
        quote: "Organize teams around functionality. Don't separate designers from coders, testers from data modelers. Build teams the way you build code.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 140,
        quote: "Don't be a slave to formal methods. Don't blindly adopt any technique without putting it into the context of your development practices and capabilities.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 139,
        quote: "Start when you're ready. You've been building experience all your life. don't ignore niggling doubts.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 138,
        quote: "Use a project glossary. Create and maintain a single source of all the specific terms and vocabulary for a project.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 137,
        quote: "Work with a user to think like a user. It's the best way to gain insight into how the system will really be used.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 136,
        quote: "Don't use wizard code you don't understand. Wizards can generate reams of code. Make sure you understand all of it before you incorporate it into your project.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 135,
        quote: "Design to test. Start thinking about testing before you write a line of code.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 134,
        quote: "Test your estimates. Mathematical analysis of algorithms doesn't tell you everything. Try timing your code in its target environment.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 133,
        quote: "Don't program by coincidence. Rely only on reliable things. Beware of accidental complexity, and don't confuse a happy coincidence with a purposeful plan.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 132,
        quote:  "Separate views from models. Gain flexibility at low cost by designing your application in terms of models and views.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 131,
        quote: "Design using services. Design in terms of services-independent, concurrent objects behind well-defined, consistent interfaces.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 130,
        quote: "Put abstractions in code, details in metadata. Program for the general case, and put the specifics outside the compiled code base.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 129,
        quote: "Minimize coupling between modules. Avoid coupling by writing \"shy\" code and applying the Law of Demeter.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 128,
        quote: "Use exceptions for exceptional problems. Exceptions can suffer from all the readability and maintainability problems of classic spaghetti code. Reserve exceptions for exceptional things.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 127,
        quote: "Crash early. A dead program normally does a lot less damage than a crippled one.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 126,
        quote: "You can't write perfect software. Software can't be perfect. Protect your code and users from the inevitable errors.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 125,
        quote:  "Learn a text manipulation language. You spend a large part of each day working with text. Why not have the computer do some of it for you?",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 124,
        quote: "\"select\" isn't broken. It is rare to find a bug in the OS or the compiler, or even a third-party product or library. The bug is most likely in the application.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 123,
        quote: "Fix the problem, not the blame. It doesn't really matter whether the bug is your fault or someone else's - it is still your problem, and it still needs to be fixed.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 122,
        quote: "Use a single editor well. The editor should be an extension of your hand; make sure your editor is configurable, extensible, and programmable.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 121,
        quote: "Keep knowledge in plain text. Plain text won't become obsolete. It helps leverage your work and simplifies debugging and testing.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 120,
        quote: "Estimate to avoid surprises. Estimate before you start. You'll spot potential problems up front.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 119,
        quote: "Prototype to learn. Prototyping is a learning experience. Its value lies not in the code you produce, but in the lessons you learn.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 118,
        quote: "There are no final decisions. No decision is cast in stone. Instead, consider each as being written in the sand at the beach, and plan for change.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 117,
        quote: "Make it easy to reuse. If it's easy to reuse, people will. Create an environment that supports reuse.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 116,
        quote: "It's both what you say and the way you say it. There's no point in having great ideas if you don't communicate them effectively.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 115,
        quote: "Invest regularly in your knowledge portfolio. Make learning a habit.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 114,
        quote: "Remember the big picture. don't get so engrossed in the details that you forget to check what's happening around you.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 113,
        quote: "Don't live with broken windows. Fix bad designs, wrong decisions, and poor code when you see them.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 112,
        quote: "Think about your work. Turn off the autopilot and take control. Constantly critique and appraise your work.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 111,
        quote: "Gently exceed your users' expectations. Come to understand your users' expectations, then deliver just that little bit more.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 110,
        quote: "English is just a programming language. Write documents as you would write code: honor the DRY principle, use metadata, MVC, automatic generation, and so on.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 109,
        quote: "Test state coverage, not code coverage. Identify and test significant program states. Just testing lines of code isn't enough.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 108,
        quote: "Coding ain't done \"til all the Tests run.\" Nuff said.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 107,
        quote: "Don't use manual procedures. A shell script or batch file will execute the same instructions, in the same order, time after time.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 106,
        quote: "Costly tools don't produce better designs. Beware of vendor hype, industry dogma, and the aura of the price tag. Judge tools on their merits.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 105,
        quote: "Some things are better done than described. Don't fall into the specification spiral - at some point you need to start coding.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 104,
        quote: "Don't think outside the box - find the box. When faced with an impossible problem, identify the real constraints. Ask yourself: \"Does it have to be done this way? Does it have to be done at all?\"",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 103,
        quote: "Abstractions live longer than details. Invest in the abstraction, not the implementation. Abstractions can survive the barrage of changes from different implementations and new technologies.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 102,
        quote: "Don't gather requirements - dig for them. Requirements rarely lie on the surface. They're buried deep beneath layers of assumptions, misconceptions, and politics.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 101,
        quote: "Test your software, or your users will. Test ruthlessly. don't make your users find bugs for you.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 100,
        quote: "Refactor early, refactor often. Just as you might weed and rearrange a garden, rewrite, rework, and re-architect code when it needs it. Fix the root of the problem.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 99,
        quote: "Estimate the order of your algorithms. Get a feel for how long things are likely to take before you write code.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 98,
        quote: "Use blackboards to coordinate workflow. Use blackboards to coordinate disparate facts and agents, while maintaining independence and isolation among participants.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 97,
        quote: "Always design for concurrency. Allow for concurrency, and you'll design cleaner interfaces with fewer assumptions.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 96,
        quote: "Analyze workflow to improve concurrency. Exploit concurrency in your user's workflow.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 95,
        quote: "Configure, don't integrate. Implement technology choices for an application as configuration options, not through integration or engineering.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 94,
        quote: "Finish what you start. Where possible, the routine or object that allocates a resource should be responsible for deallocating it.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 93,
        quote: "Use assertions to prevent the impossible. Assertions validate your assumptions. Use them to protect your code from an uncertain world.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 92,
        quote: "Design With contracts. Use contracts to document and verify that code does no more and no less than it claims to do.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 91,
        quote: "Write code that writes code. Code generators increase your productivity and help avoid duplication.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 90,
        quote: "Don't assume it - prove it. Prove your assumptions in the actual environment - with real data and boundary conditions.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 89,
        quote: "Don't panic when debugging Take a deep breath and THINK! about what could be causing the bug.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 88,
        quote: "Always use source code control. Source code control is a time machine for your work - you can go back.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 87,
        quote: "Use the power of command shells. Use the shell when graphical user interfaces don't cut it.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 86,
        quote: "Iterate the schedule with the code. Use experience you gain as you implement to refine the project time scales.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 85,
        quote: "Program close to the problem domain. Design and code in your user's language.",
        permalink:Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 84,
        quote: "Use tracer bullets to find the target. Tracer bullets let you hone in on your target by trying things and seeing how close they land.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 83,
        quote: "Eliminate effects between unrelated things. Design components that are self-contained, independent, and have a single, well-defined purpose.",
        permalink:Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 82,
        quote: "DRY - don't Repeat Yourself. Every piece of knowledge must have a single, unambiguous, authoritative representation within a system.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 81,
        quote: "Critically analyze what you read and hear. don't be swayed by vendors, media hype, or dogma. Analyze information in terms of you and your project.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 80,
        quote: "Make quality a requirements issue. Involve your users in determining the project's real quality requirements.",
        permalink:  Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 79,
        quote: "Be a catalyst for change. You can't force change on people. Instead, show them how the future might be and help them participate in creating it.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 78,
        quote:  "Provide options, don't make lame excuses. Instead of excuses, provide options. don't say it can't be done; explain what can be done.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 77,
        quote: "Care about your craft. Why spend your life developing software unless you care about doing it well?",
        permalink:Origin::VimStartify
    },
    Quote {
        author: "Andrew Gerrand",
        id: 76,
        quote: "Methods are just functions with a special first argument.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Rob Pike",
        id: 75,
        quote: "Fancy algorithms are slow when n is small, and n is usually small.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Larry Wall",
        id: 74,
        quote: "Almost every programming language is overrated by its practitioners.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 73,
        quote: "Computers are harder to maintain at high altitude. Thinner air means less cushion between disk heads and platters. Also more radiation.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Bartosz Milewski",
        id: 72,
        quote: "If programmers were electricians, parallel programmers would be bomb disposal experts. Both cut wires.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Cooper",
        id: 71,
        quote: "All idioms must be learned. Good idioms only need to be learned once.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Richard Feynman",
        id: 70,
        quote: "For a successful technology, reality must take precedence over public relations, for Nature cannot be fooled.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 69,
        quote: "All loops are infinite ones for faulty RAM modules.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Rob Pike",
        id: 68,
        quote: "\"dd\" is horrible on purpose. It's a joke about OS/360 JCL. But today it's an internationally standardized joke. I guess that says it all.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Edsger W. Dijkstra",
        id: 67,
        quote:
            "Abstraction is not about vagueness, it is about being precise at a new semantic level.",
        permalink:
            Origin::VimStartify
    },
    Quote {
        author: "Rob Pike",
        id: 66,
        quote: "Caches are bugs waiting to happen.",
        permalink:
            Origin::VimStartify
    },
    Quote {
        author: "Leslie Lamport",
        id: 65,
        quote: "If you don't start with a spec, every piece of code you write is a patch.",
        permalink:
            Origin::VimStartify
    },
    Quote {
        author: "Frank Wilczek",
        id: 64,
        quote: "If you don't make mistakes, you're not working on hard enough problems.",
        permalink:
            Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 63,
        quote: "Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 62,
        quote: "Contrary to popular belief, Unix is user friendly. It just happens to be very selective about who it decides to make friends with.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 61,
        quote: "Unix was not designed to stop its users from doing stupid things, as that would also stop them from doing clever things.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "John Woods",
        id: 60, // Almost identical to 9 however it sounds slightly more functional
        quote: "Always code as if the person who ends up maintaining your code is a violent psychopath who knows where you live.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Rich Hickey",
        id: 59,
        quote: "Patterns mean \"I have run out of language.\"",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Doug Linder",
        id: 58,
        quote: "A good programmer is someone who always looks both ways before crossing a one-way street.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Phil Wadler",
        id: 57,
        quote: "The essence of XML is this: the problem it solves is not hard, and it does not solve the problem well.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Kay",
        id: 56,
        quote: "Progress in a fixed context is almost always a form of optimization. Creative acts generally don't stay in the context that they are in.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Edsger W. Dijkstra",
        id: 55,
        quote: "The computing scientist's main challenge is not to get confused by the complexities of his own making.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 54,
        quote: "If a system is to serve the creative spirit, it must be entirely comprehensible to a single individual.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Vincent Van Gogh",
        id: 53,
        quote: "I would rather die of passion than of boredom.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Glyn Williams",
        id: 52,
        quote: "I think a lot of new programmers like to use advanced data structures and advanced language features as a way of demonstrating their ability. I call it the lion-tamer syndrome. Such demonstrations are impressive, but unless they actually translate into real wins for the project, avoid them.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Kay",
        id: 51,
        quote: "If you don't fail at least 90% of the time, you're not aiming high enough.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Peter Drucker",
        id: 50,
        quote: "There is nothing quite so useless as doing with great efficiency something that should not be done at all.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 49,
        quote: "It is better to have 100 functions operate on one data structure than 10 functions on 10 data structures.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 48,
        quote: "Recursion is the root of computation since it trades description for time.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 47,
        quote: "Optimization hinders evolution.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 46,
        quote: "Simplicity does not precede complexity, but follows it",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 45,
        quote: "It is easier to change the specification to fit the program than vice versa.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 44,
        quote: "Fools ignore complexity. Pragmatists suffer it. Some can avoid it. Geniuses remove it.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Alan Perlis",
        id: 43,
        quote: "Adapting old programs to fit new machines usually means adapting new machines to behave like old ones.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Unknown",
        id: 42,
        quote: "If you don't finish then you're just busy, not productive.",
        permalink: Origin::VimStartify
    },
    Quote {
        author: "Bill Sempf",
        id: 41,
        quote: "QA Engineer walks into a bar. Orders a beer. Orders 0 beers. Orders 999999999 beers. Orders a lizard. Orders -1 beers. Orders a sfdeljknesv.",
        permalink: Origin::StormConsultancy(44)
    },
    Quote {
        author: "Phil Karlton",
        id: 40,
        quote: "There are only two hard things in Computer Science: cache invalidation, naming things and off-by-one errors.",
        permalink: Origin::StormConsultancy(43)
    },
    Quote {
        author: "Jeff Atwood",
        id: 39,
        quote: "In software, we rarely have meaningful requirements. Even if we do, the only measure of success that matters is whether our solution solves the customer's shifting idea of what their problem is.",
        permalink: Origin::StormConsultancy(42)
    },
    Quote {
        author: "Robert Sewell",
        id: 38,
        quote: "If Java had true garbage collection, most programs would delete themselves upon execution.",
        permalink: Origin::StormConsultancy(41)
    },
    Quote {
        author: "Gavin Russell Baker",
        id: 37,
        quote: "C++ : Where friends have access to your private members.",
        permalink: Origin::StormConsultancy(40)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 36,
        quote: "In C++ it's harder to shoot yourself in the foot, but when you do, you blow off your whole leg.",
        permalink: Origin::StormConsultancy(39)
    },
    Quote {
        author: "Larry DeLuca",
        id: 35,
        quote: "I've noticed lately that the paranoid fear of computers becoming intelligent and taking over the world has almost entirely disappeared from the common culture.  Near as I can tell, this coincides with the release of MS-DOS.",
        permalink: Origin::StormConsultancy(37)
    },
    Quote {
        author: "Mark Gibbs",
        id: 34,
        quote: "No matter how slick the demo is in rehearsal, when you do it in front of a live audience, the probability of a flawless presentation is inversely proportional to the number of people watching, raised to the power of the amount of money involved.",
        permalink: Origin::StormConsultancy(36)
    },
    Quote {
        author: "Henry Petroski",
        id: 33,
        quote: "The most amazing achievement of the computer software industry is its continuing cancellation of the steady and staggering gains made by the computer hardware industry.",
        permalink: Origin::StormConsultancy(35)
    },
    Quote {
        author: "Jeremy S. Anderson",
        id: 32,
        quote: "There are two major products that come out of Berkeley: LSD and UNIX.  We don't believe this to be a coincidence.",
        permalink: Origin::StormConsultancy(34)
    },
    Quote {
        author: "Sam Ewing",
        id: 31,
        quote: "Computers are like bikinis. They save people a lot of guesswork.",
        permalink: Origin::StormConsultancy(33)
    },
    Quote {
        author: "Jamie Zawinski",
        id: 30,
        quote: "Linux is only free if your time has no value.",
        permalink: Origin::StormConsultancy(32)
    },
    Quote {
        author: "Dick Brandon",
        id: 29,
        quote: "Documentation is like sex; when it's good, it's very, very good, and when it's bad, it's better than nothing.",
        permalink: Origin::StormConsultancy(31)
    },
    Quote {
        author: "Richard Moore",
        id: 28,
        quote: "The difference between theory and practice is that in theory, there is no difference between theory and practice.",
        permalink: Origin::StormConsultancy(30)
    },
    Quote {
        author: "Michael Sinz",
        id: 27,
        quote: "Programming is like sex: one mistake and you're providing support for a lifetime.",
        permalink: Origin::StormConsultancy(29)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 26,
        quote: "There are only two kinds of programming languages: those people always bitch about and those nobody uses.",
        permalink: Origin::StormConsultancy(28)
    },
    Quote {
        author: "Donald Knuth",
        id: 25,
        quote: "Beware of bugs in the above code; I have only proved it correct, not tried it.",
        permalink: Origin::StormConsultancy(27)
    },
    Quote {
        author: "Tom Van Vleck",
        id: 24,
        quote: "We know about as much about software quality problems as they knew about the Black Plague in the 1600s. We've seen the victims' agonies and helped burn the corpses. We don't know what causes it; we don't really know if there is only one disease. We just suffer \u{2014} and keep pouring our sewage into our water supply.",
        permalink: Origin::StormConsultancy(26)
    },
    Quote {
        author: "N.J. Rubenking",
        id: 23,
        quote: "Writing the first 90 percent of a computer program takes 90 percent of the time. The remaining ten percent also takes 90 percent of the time and the final touches also take 90 percent of the time.",
        permalink: Origin::StormConsultancy(25)
    },
    Quote {
        author: "C.A.R. Hoare",
        id: 22,
        quote: "There are two ways of constructing a software design; one way is to make it so simple that there are obviously no deficiencies, and the other way is to make it so complicated that there are no obvious deficiencies. The first method is far more difficult.",
        permalink: Origin::StormConsultancy(24)
    },
    Quote {
        author: "James O. Coplien",
        id: 21,
        quote: "You should name a variable using the same care with which you name a first-born child.",
        permalink: Origin::StormConsultancy(23)
    },
    Quote {
        author: "Fred Brooks",
        id: 20,
        quote: "Einstein argued that there must be simplified explanations of nature, because God is not capricious or arbitrary. No such faith comforts the software engineer.",
        permalink: Origin::StormConsultancy(22)
    },
    Quote {
        author: "Unknown",
        id: 19,
        quote: "XML is like violence - if it doesn't solve your problems, you are not using enough of it.",
        permalink: Origin::StormConsultancy(21)
    },
    Quote {
        author: "Unknown",
        id: 18,
        quote: "Saying that Java is good because it works on all platforms is like saying anal sex is good because it works on all genders.",
        permalink: Origin::StormConsultancy(20)
    },
    Quote {
        author: "Douglas Adams",
        id: 17,
        quote: "I love deadlines. I like the whooshing sound they make as they fly by.",
        permalink: Origin::StormConsultancy(19)
    },
    Quote {
        author: "Keith Bostic",
        id: 16,
        quote: "Perl - The only language that looks the same before and after RSA encryption.",

        permalink: Origin::StormConsultancy(18)
    },
    Quote {
        author: "Albert Einstein",
        id: 15,
        quote: "Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.",
        permalink: Origin::StormConsultancy(17)
    },
    Quote {
        author: "Yogi Berra",
        id: 14,
        quote: "In theory, theory and practice are the same. In practice, they're not.",
        permalink: Origin::StormConsultancy(16)
    },
    Quote {
        author: "E. W. Dijkstra",
        id: 13,
        quote:  "It is practically impossible to teach good programming style to students that have had prior exposure to BASIC. As potential programmers, they are mentally mutilated beyond hope of regeneration.",
        permalink: Origin::StormConsultancy(15)
    },
    Quote {
        author: "E. W. Dijkstra",
        id: 12,
        quote:  "If debugging is the process of removing software bugs, then programming must be the process of putting them in.",
        permalink: Origin::StormConsultancy(14)
    },
    Quote {
        author: "Mitch Ratcliffe",
        id: 11,
        quote:  "A computer lets you make more mistakes faster than any other invention in human history, with the possible exceptions of handguns and tequila.",
        permalink: Origin::StormConsultancy(13)
    },
    Quote {
        author: "Bjarne Stroustrup",
        id: 10,
        quote:  "I have always wished for my computer to be as easy to use as my telephone; my wish has come true because I can no longer figure out how to use my telephone.",
        permalink: Origin::StormConsultancy(12)
    },
    Quote {
        author: "Ovidiu Platon",
        id: 9,
        quote: "I don't care if it works on your machine! We are not shipping your machine!",
        permalink: Origin::StormConsultancy(11)
    },
    Quote {
        author: "Rich Cook, The Wizardry Compiled",
        id: 8,
        quote:  "Programming today is a race between software engineers striving to build bigger and better idiot-proof programs, and the Universe trying to produce bigger and better idiots. So far, the Universe is winning.",
        permalink: Origin::StormConsultancy(10)
    },
    Quote {
        author: "Rick Osborne",
        id: 7,
        quote: "Always code as if the guy who ends up maintaining your code will be a violent psychopath who knows where you live.",
        permalink: Origin::StormConsultancy(9)
    },
    Quote {
        author: "Charles Babbage",
        id: 6,
        quote: "On two occasions I have been asked, \"Pray, Mr. Babbage, if you put into the machine wrong figures, will the right answers come out?' I am not able rightly to apprehend the kind of confusion of ideas that could provoke such a question.\"",
        permalink: Origin::StormConsultancy(8)
    },
    Quote {
        author: "Jon Ribbens",
        id: 5,
        quote: "PHP is a minor evil perpetrated and created by incompetent amateurs, whereas Perl is a great and insidious evil, perpetrated by skilled but perverted professionals.",
        permalink: Origin::StormConsultancy(7)
    },
    Quote {
        author: "Brian Kernighan",
        id: 4,
        quote: "Debugging is twice as hard as writing the code in the first place. Therefore, if you write the code as cleverly as possible, you are, by definition, not smart enough to debug it.",
        permalink: Origin::StormConsultancy(5)
    },
    Quote {
        author: "Jamie Zawinski",
        id: 3,
        quote: "Some people, when confronted with a problem, think \"I know, I'll use regular expressions.\" Now they have two problems.",
        permalink: Origin::StormConsultancy(4)
    },
    Quote {
        author: "Hofstadter's Law",
        id: 2,
        quote: "It always takes longer than you expect, even when you take into account Hofstadter's Law.",
        permalink: Origin::StormConsultancy(3)
    },
    Quote {
        author: "C.A.R. Hoare",
        id: 1,
        quote: "We should forget about small efficiencies, say about 97% of the time: premature optimization is the root of all evil.",
        permalink: Origin::StormConsultancy(1)
    }
];
