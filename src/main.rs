use std::{fs, path::Path};

use color_eyre::eyre::{eyre, Result, WrapErr};
use gix::Repository;
use nom::{branch::alt, bytes::complete::take_until};

fn main() -> Result<()> {
    let dest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/this-week-in-rust"));

    println!("Setting up repository...");
    let _repo = git_clone_or_open("https://github.com/rust-lang/this-week-in-rust.git", dest)?;

    let mut collection = Vec::new();

    println!("Collecting quotes of the week...");
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

        let content = fs::read_to_string(&path)?;

        match extract_quote_of_the_week(&content)? {
            Some(quote) => collection.push((path, Some(quote.to_string()))),

            None => collection.push((path, None)),
        };
    }

    let total = collection.len();
    println!(
        "Found {} quotes of the week in {total} editions:",
        collection
            .iter()
            .filter(|(_, maybe)| maybe.is_some())
            .count()
    );
    println!("{collection:#?}");

    Ok(())
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

/// Skip all text until the start-of-quote marker.
///
/// Returns `None` if start-of-quote marker was not found.
fn find_quote(input: &str) -> Option<&str> {
    match take_until::<&str, &str, nom::error::Error<&str>>("# Quote of the Week\n\n")(input) {
        Ok((input, _)) => Some(input),
        Err(_) => None, // TakeUntil error -> Quote not found.
    }
}

/// Take all text until one of three canonical end-of-quote markers.
///
/// Returns an error if no end-of-quote marker was found.
fn take_quote(input: &str) -> Result<&str> {
    match alt((
        take_until::<&str, &str, nom::error::Error<&str>>("[Please submit"),
        take_until::<&str, &str, nom::error::Error<&str>>("[Submit"),
        take_until::<&str, &str, nom::error::Error<&str>>("\n\n# "),
    ))(input)
    {
        Ok((_, quote)) => Ok(quote),
        Err(err) => Err(eyre!("failed to find end of quote: {err:?}")),
    }
}

/// Try to open repository at `dest`, if directory exists,
/// otherwise clone from remote url `src` into local directory `dest`.
fn git_clone_or_open(src: &str, dest: &Path) -> Result<Repository> {
    // SAFETY: The closure doesn't use mutexes or memory allocation, so it should be safe to call from a signal handler.
    unsafe {
        gix::interrupt::init_handler(1, || {})?;
    }

    if dest.is_dir() {
        println!("Repository at '{}' is present. Opening...", dest.display());
        gix::open(dest)
            .wrap_err_with(|| format!("failed to open repository at '{}'", dest.display()))
    } else {
        git_clone(src, dest)
            .wrap_err_with(|| format!("failed cloning '{src}' into '{}'", dest.display()))
    }
}

/// Clone git repository from remote url `src` into local directory `dest`.
fn git_clone(src: &str, dest: &Path) -> Result<Repository> {
    let url = gix::url::parse(src.into())?;

    println!("Cloning {url:?} into '{}'...", dest.display());
    let (mut prepare_checkout, _) = gix::prepare_clone(url, dest)?
        .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

    println!(
        "Checking out into {:?} ...",
        prepare_checkout.repo().work_dir().expect("should be there")
    );

    let (repo, _) =
        prepare_checkout.main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)?;

    let remote = repo
        .find_default_remote(gix::remote::Direction::Fetch)
        .expect("always present after clone")?;

    println!(
        "Default remote: {} -> {}",
        remote
            .name()
            .expect("default remote is always named")
            .as_bstr(),
        remote
            .url(gix::remote::Direction::Fetch)
            .expect("should be the remote URL")
            .to_bstring(),
    );

    Ok(repo)
}
