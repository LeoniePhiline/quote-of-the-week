//! # This Week in Rust: _Quote of the Week_
//!
//! An ad-hoc, just-for-fun project
//! of [Rust Hack and Learn Meetup Berlin](https://berline.rs/).

use std::{fs, path::Path};

use chrono::NaiveDate;
use color_eyre::eyre::{eyre, Result, WrapErr};
use gix::sec::trust::DefaultForLevel;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::map_res,
    Parser,
};

fn main() -> Result<()> {
    // Prepare repository
    let dest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/this-week-in-rust"));

    println!("Setting up repository...");
    git_clone_or_open("https://github.com/rust-lang/this-week-in-rust.git", dest)?;

    // Walk repository and parse files
    println!("Collecting quotes of the week...");
    let mut collection = Vec::new();

    for entry in fs::read_dir(dest.join("content"))? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let Some(ext) = path.extension() else {
            continue;
        };

        let Some(ext) = ext.to_str() else {
            continue;
        };

        if !matches!(ext, "md" | "markdown") {
            continue;
        }

        // Read markdown file
        let content = fs::read_to_string(&path)?;

        // Extract edition date from file name
        let date = parse_date(
            path.file_name()
                .ok_or_else(|| eyre!("path '{}' had no file name", path.display()))?
                .to_str()
                .ok_or_else(|| eyre!("path '{}' had non UTF-8 file name", path.display()))?,
        )?;

        // Extract quote of the week,
        // if present in edition content
        match extract_quote_of_the_week(&content)? {
            Some(quote) => collection.push((date, Some(quote.to_string()))),

            None => collection.push((date, None)),
        };
    }

    // Sort results
    collection.sort_unstable_by_key(|entry| entry.0);

    // Print results
    let total = collection.len();
    println!(
        "Found {} quotes of the week in {total} editions:",
        collection
            .iter()
            .filter(|(_, maybe)| maybe.is_some())
            .count()
    );

    println!("# Quotes of the week\n");
    for (date, maybe_quote) in collection {
        if let Some(quote) = maybe_quote {
            println!("## {date} - Quote of the Week\n");
            println!("{quote}\n\n");
        } else {
            println!("## {date}\n");
            println!("_No Quote of the Week._\n");
        }
    }

    Ok(())
}

/// Parse a chrono naive date without timezone
/// from the provided (this week in rust edition file name) string slice.
///
/// Returns an error if the input does not start with a date in ISO format "YYYY-MM-DD".
fn parse_date(input: &str) -> Result<chrono::NaiveDate> {
    let (_, (year, _, month, _, day)) = (
        map_res(digit1, str::parse::<i32>),
        tag("-"),
        map_res(digit1, str::parse::<u32>),
        tag("-"),
        map_res(digit1, str::parse::<u32>),
    )
        .parse(input)
        .map_err(|err: nom::Err<nom::error::Error<&str>>| {
            eyre!("failed to match date from input '{input}': {err:#?}")
        })?;

    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| eyre!("not a valid date: '{year}-{month}-{day}'"))
}

/// Extracts the quote from the input string slice.
///
/// Returns `None` if no quote could be found.
///
/// Returns an error if a quote was found, but it was not terminated.
fn extract_quote_of_the_week(input: &str) -> Result<Option<&str>> {
    let Some(input) = find_quote(input) else {
        return Ok(None);
    };

    Ok(Some(take_quote(input)?))
}

/// Skip all text until and including the start-of-quote marker.
///
/// Returns `None` if start-of-quote marker was not found.
fn find_quote(input: &str) -> Option<&str> {
    let start = "# Quote of the Week\n\n";
    let start_alt = "# Quotes of the Week\n\n";

    // Skip all text until the start-of-quote marker.
    let Ok((input, _)) = alt((
        take_until::<&str, &str, nom::error::Error<&str>>(start),
        take_until::<&str, &str, nom::error::Error<&str>>(start_alt),
    ))
    .parse(input) else {
        return None; // TakeUntil error -> Quote not found.
    };

    // Consume the marker, removing it from the parseable remainder.
    let (input, _) = alt((
        tag::<&str, &str, nom::error::Error<&str>>(start),
        tag::<&str, &str, nom::error::Error<&str>>(start_alt),
    ))
    .parse(input)
    .unwrap();

    Some(input)
}

/// Take all text until one of three canonical end-of-quote markers.
///
/// Returns an error if no end-of-quote marker was found.
fn take_quote(input: &str) -> Result<&str> {
    match alt((
        take_until("[Please submit"),
        take_until("[Submit"),
        take_until("\n\n# "),
    ))
    .parse(input)
    {
        Ok((_, quote)) => Ok(quote.trim()),
        Err::<_, nom::Err<nom::error::Error<&str>>>(err) => {
            Err(eyre!("failed to find end of quote: {err:?}"))
        }
    }
}

/// Try to open repository at `dest`, if directory exists,
/// otherwise clone from remote url `src` into local directory `dest`.
fn git_clone_or_open(src: &str, dest: &Path) -> Result<()> {
    // SAFETY: The closure doesn't use mutexes or memory allocation,
    //         so it should be safe to call from a signal handler.
    #[allow(unsafe_code)]
    unsafe {
        gix::interrupt::init_handler(1, || {})?;
    }

    if dest.is_dir() {
        println!("Repository at '{}' is present. Opening...", dest.display());
        gix::open(dest)
            .map(|_| ())
            .wrap_err_with(|| format!("failed to open repository at '{}'", dest.display()))
    } else {
        git_clone(src, dest)
            .wrap_err_with(|| format!("failed cloning '{src}' into '{}'", dest.display()))
    }
}

/// Clone git repository from remote url `src` into local directory `dest`.
fn git_clone(src: &str, dest: &Path) -> Result<()> {
    let url = gix::url::parse(src.into())?;

    println!("Cloning '{src}' into '{}'...", dest.display());
    let (mut prepare_checkout, _) = gix::clone::PrepareFetch::new(
        url,
        dest,
        gix::create::Kind::WithWorktree,
        gix::create::Options {
            destination_must_be_empty: true,
            ..Default::default()
        },
        gix::open::Options::default_for_level(gix::sec::Trust::Reduced),
    )
    .wrap_err("failed to prepare clone")?
    .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
    .wrap_err("failed to fetch, then checkout")?;

    println!(
        "Checking out into {:?} ...",
        prepare_checkout.repo().workdir().expect("should be there")
    );

    prepare_checkout.main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use color_eyre::eyre::{OptionExt, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::{extract_quote_of_the_week, find_quote, git_clone_or_open, parse_date, take_quote};

    use super::git_clone;

    #[test]
    fn clones_repo() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let dest = temp_dir.path().join("this-week-in-rust");

        git_clone("https://github.com/rust-lang/this-week-in-rust.git", &dest)?;

        assert_eq!(dest.join(".git").is_dir(), true);

        Ok(())
    }

    #[test]
    fn refuses_to_clone_if_dir_exists() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let dest = temp_dir.path().join("this-week-in-rust");

        std::fs::create_dir(&dest)?;
        std::fs::File::create_new(dest.join("not-empty"))?;

        let err =
            git_clone("https://github.com/rust-lang/this-week-in-rust.git", &dest).unwrap_err();

        assert_eq!(
            err.root_cause()
                .to_string()
                .starts_with("Refusing to initialize the non-empty directory as"),
            true
        );

        Ok(())
    }

    #[test]
    fn opens_existing_repo() -> Result<()> {
        // Opening self not fail.
        git_clone_or_open("<invalid>", &std::env::current_dir()?)?;

        Ok(())
    }

    #[test]
    fn parses_valid_date() -> Result<()> {
        let date = parse_date("2024-03-29-foo-bar")?;

        assert_eq!(
            date,
            NaiveDate::from_ymd_opt(2024, 3, 29).ok_or_eyre("2024-03-29 is a valid date")?
        );

        Ok(())
    }

    #[test]
    fn fails_parsing_invalid_date() {
        let err = parse_date("2024-03-no-day-foo-bar").unwrap_err();

        assert_eq!(
            err.root_cause().to_string(),
            indoc! { r#"
                failed to match date from input '2024-03-no-day-foo-bar': Error(
                    Error {
                        input: "no-day-foo-bar",
                        code: Digit,
                    },
                )"# }
        );
    }

    #[test]
    fn fails_creating_invalid_date() {
        let err = parse_date("2024-03-32-foo-bar").unwrap_err();

        assert_eq!(
            err.root_cause().to_string(),
            "not a valid date: '2024-3-32'"
        );
    }

    /// "Quote" without 's'.
    #[test]
    fn finds_quote_of_the_week_heading() {
        let rest = find_quote(indoc! { r"
            # This Week in Rust

            Some text.

            # Quote of the Week

            > A quote

            More text.
        " });

        assert_eq!(
            rest,
            Some(indoc! { "
            > A quote

            More text.
            "})
        );
    }

    /// "Quotes" with 's'.
    #[test]
    fn finds_quotes_of_the_week_heading() {
        let rest = find_quote(indoc! { r"
            # This Week in Rust

            Some text.

            # Quotes of the Week

            > A quote

            More text.
        " });

        assert_eq!(
            rest,
            Some(indoc! { "
            > A quote

            More text.
            "})
        );
    }

    /// "Quotes" with 's'.
    #[test]
    fn passes_if_no_quotes_of_the_week_heading() {
        let rest = find_quote(indoc! { r"
            # This Week in Rust

            Some text.

            # Some other heading

            More text.
        " });

        assert_eq!(rest, None);
    }

    #[test]
    fn takes_quote_trimmed_from_remainder_with_next_heading() -> Result<()> {
        let quote = take_quote(indoc! { r"

            > The quote.

            By some author

            # Next heading
        " })?;

        assert_eq!(
            quote,
            indoc! { r"
            > The quote.

            By some author"}
        );

        Ok(())
    }

    #[test]
    fn takes_quote_trimmed_from_remainder_with_call_to_action() -> Result<()> {
        let quote = take_quote(indoc! { r"

            > The quote.

            By some author

            [Submit your quotes for next week][submit]!
        " })?;

        assert_eq!(
            quote,
            indoc! { r"
            > The quote.

            By some author"}
        );

        Ok(())
    }

    #[test]
    fn takes_quote_trimmed_from_remainder_with_friendly_call_to_action() -> Result<()> {
        let quote = take_quote(indoc! { r"

            > The quote.

            By some author

            [Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)
        " })?;

        assert_eq!(
            quote,
            indoc! { r"
            > The quote.

            By some author"}
        );

        Ok(())
    }

    #[test]
    fn extracts_quote_trimmed() -> Result<()> {
        let quote = extract_quote_of_the_week(indoc! { r"
            # This Week in Rust

            Some text.

            # Quote of the Week

            > A quote

            More text.

            [Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)
        " })?;

        assert_eq!(
            quote,
            Some(indoc! { "
            > A quote

            More text."})
        );

        Ok(())
    }
}
