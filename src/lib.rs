//! # FuzzyDate: Date Input for Humans
//!
//! A Parser which can turn a variety of input strings into a DateTime
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! fuzzydate = "0.1"
//! ```
//!
//! ## Example
//!
//! ```rust
//! use fuzzydate::parse;
//! use chrono::NaiveDateTime;
//!
//! fn main() {
//!     let date_string = "Five days after 2/12/22 5:00 PM";
//!     let date = parse(date_string).unwrap();
//!     println!("{:?}", date);
//! }
//! ```
//!
//! Any relevant date time information not specified is assumed to be
//! the value of the current date time.
//!
//! ## Grammar
//! ```text
//! <datetime> ::= <time>
//!              | <date> <time>
//!              | <date> , <time>
//!              | <duration> after <datetime>
//!              | <duration> from <datetime>
//!              | <duration> before <datetime>
//!              | <duration> ago
//!              | now
//!
//! <article> ::= a
//!            | an
//!            | the
//!
//! <date> ::= today
//!          | tomorrow
//!          | yesterday
//!          | <num> / <num> / <num>
//!          | <num> - <num> - <num>
//!          | <month> <num> <num>
//!          | <month> <num> , <num>
//!          | <relative_specifier> <unit>
//!          | <relative_specifier> <weekday>
//!          | <weekday>
//!
//! <relative_specifier> ::= this
//!                        | next
//!                        | last
//!
//! <weekday> ::= monday
//!             | tuesday
//!             | wednesday
//!             | thursday
//!             | friday
//!             | saturday
//!             | sunday
//!             | mon
//!             | tue
//!             | wed
//!             | thu
//!             | fri
//!             | sat
//!             | sun
//!
//! <month> ::= january
//!           | february
//!           | march
//!           | april
//!           | may
//!           | june
//!           | july
//!           | august
//!           | september
//!           | october
//!           | november
//!           | december
//!           | jan
//!           | feb
//!           | mar
//!           | apr
//!           | jun
//!           | jul
//!           | aug
//!           | sep
//!           | oct
//!           | nov
//!           | dec
//!
//! <duration> ::= <num> <unit>
//!              | <article> <unit>
//!
//! <time> ::= <num>:<num>
//!          | <num>:<num> am
//!          | <num>:<num> pm
//!          |
//!
//! <unit> ::= day
//!          | days
//!          | week
//!          | weeks
//!          | hour
//!          | hours
//!          | minute
//!          | minutes
//!          | min
//!          | mins
//!          | month
//!          | months
//!          | year
//!          | years
//!
//! <num> ::= <num_triple> <num_triple_unit> and <num>
//!         | <num_triple> <num_triple_unit> <num>
//!         | <num_triple> <num_triple_unit>
//!         | <num_triple_unit> and <num>
//!         | <num_triple_unit> <num>
//!         | <num_triple_unit>
//!         | <num_triple>
//!         | NUM   ; number literal greater than or equal to 1000
//!
//! <num_triple> ::= <ones> hundred and <num_double>
//!                | <ones> hundred <num_double>
//!                | <ones> hundred
//!                | hundred and <num_double>
//!                | hundred <num_double>
//!                | hundred
//!                | <num_double>
//!                | NUM    ; number literal less than 1000 and greater than 99
//!
//! <num_triple_unit> ::= thousand
//!                     | million
//!                     | billion
//!
//! <num_double> ::= <ones>
//!                | <tens> - <ones>
//!                | <tens> <ones>
//!                | <tens>
//!                | <teens>
//!                | NUM    ; number literal less than 100 and greater than 19
//!
//! <tens> ::= twenty
//!          | thirty
//!          | forty
//!          | fifty
//!          | sixty
//!          | seventy
//!          | eighty
//!          | ninety
//!
//! <teens> ::= ten
//!           | eleven
//!           | twelve
//!           | thirteen
//!           | fourteen
//!           | fifteen
//!           | sixteen
//!           | seventeen
//!           | eighteen
//!           | nineteen
//!           | NUM     ; number literal less than 20 and greater than 9
//!
//! <ones> ::= one
//!          | two
//!          | three
//!          | four
//!          | five
//!          | six
//!          | seven
//!          | eight
//!          | nine
//!          | NUM      ; number literal less than 10
//! ```

mod lexer;
mod ast;

use chrono::{NaiveDateTime, NaiveTime, Local};

/// Parse an input string into a chrono NaiveDateTime, using the default
/// values from the specified default value where not specified
pub fn parse_with_default_time(input: &str, default: NaiveTime) -> Result<NaiveDateTime, String> {
    let lexemes = lexer::Lexeme::lex_line(input.into())?;
    let tree = ast::DateTime::parse(lexemes.as_slice());

    if tree.is_none() {
        return Err("Unrecognized Date Format".into());
    }

    let date = tree.unwrap().0.to_chrono(default);
    Ok(date)
}

/// Parse an input string into a chrono NaiveDateTime with the default
/// time being now
pub fn parse(input: &str) -> Result<NaiveDateTime, String> {
    parse_with_default_time(input, Local::now().naive_local().time())
}

#[test]
fn test_parse() {
    use chrono::Datelike;
    let input = "2/12/2022";
    let date = parse(input).unwrap();

    assert_eq!(2, date.month());
    assert_eq!(12, date.day());
    assert_eq!(2022, date.year());
}

#[test]
fn test_malformed() {
    let input = "Hello World";
    let date = parse(input);
    assert!(date.is_err());
}

#[test]
fn test_empty() {
    let input = "";
    let date = parse(input);
    assert!(date.is_err());
}
