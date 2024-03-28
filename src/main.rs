use std::{fs, path::Path};

use color_eyre::eyre::{Result, WrapErr};
use gix::Repository;

fn main() -> Result<()> {
    let dest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/this-week-in-rust"));

    println!("Setting up repository...");
    let _repo = git_clone_or_open("https://github.com/rust-lang/this-week-in-rust.git", dest)?;

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

        println!("{}", path.display());
    }

    Ok(())
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
