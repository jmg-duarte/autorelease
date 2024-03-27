mod version;

use version::Bump;

use clap::Parser;
use semver::Version;
use std::path::PathBuf;

use gix::{bstr::ByteSlice, revision::walk::Info, traverse::commit::Sorting, Repository};

#[derive(Parser)]
struct App {
    /// The target repository directory. Defaults to the current directory.
    ///
    /// If the directory is not a `git` repository, its parents will be
    /// checked recursively for `git` repositories.
    #[arg(short, long = "target_dir", default_value = ".")]
    target_directory: PathBuf,

    /// Perform an empty release commit. Requires the index to be clean, use `--force` to ignore.
    ///
    /// The release commit is written as `release: <NEW_VERSION>`.
    #[arg(short = 'r', long = "release", default_value_t = false)]
    perform_release: bool,

    /// Ignore the index status.
    #[arg(short = 'f', default_value_t = false)]
    force: bool,
}

// TODO: remove the unwrap
fn find_latest_release(repository: &Repository) -> Option<Info<'_>> {
    let head = repository.head_id().unwrap();
    let mut platform = repository
        .rev_walk([head])
        .sorting(Sorting::ByCommitTimeNewestFirst)
        .all()
        .unwrap();
    while let Some(Ok(oid)) = platform.next() {
        let commit = oid.object().unwrap();
        let commit_message = commit.message().unwrap();
        if commit_message.title.starts_with_str("release:") {
            return Some(oid);
        }
    }
    None
}

fn calculate_new_version(repository: &Repository, latest_release_oid: Info) -> Version {
    let head = repository.head_id().unwrap();

    let latest_release_commit = latest_release_oid.object().unwrap();
    let latest_release_message = latest_release_commit.message().unwrap();
    let latest_release_unparsed_version = latest_release_message
        .title
        .strip_prefix("release:".as_bytes())
        .unwrap()
        .trim();

    let mut version =
        Version::parse(&String::from_utf8(latest_release_unparsed_version.to_vec()).unwrap())
            .unwrap();

    let mut major = None;
    let mut minor = None;
    let mut patch = None;

    let mut platform = repository
        .rev_walk([head])
        .sorting(Sorting::ByCommitTimeNewestFirstCutoffOlderThan {
            seconds: latest_release_oid.commit_time(),
        })
        .all()
        .unwrap();

    while let Some(Ok(oid)) = platform.next() {
        let commit = oid.object().unwrap();
        let message = commit.message_raw().unwrap();
        for line in message.split_str("\n") {
            if line.starts_with_str("feat!") {
                major = Some(());
                // No point in continuing from here
                break;
            } else if minor.is_none() && line.starts_with_str("feat") {
                // We don't check for major.is_none() because it breaks out of the loop
                minor = Some(());
            } else if minor.is_none() && patch.is_none() && line.starts_with_str("fix") {
                // We check for minor because if it is set, there is no point in increasing the patch
                patch = Some(());
            }
        }
    }

    match (major, minor, patch) {
        (Some(_), _, _) => version.bump_major(),
        (_, Some(_), _) => version.bump_minor(),
        (_, _, Some(_)) => version.bump_patch(),
        (_, _, _) => {}
    }

    version
}

fn main() {
    let mut app = App::parse();
    // Always canonicalize path
    app.target_directory = app.target_directory.canonicalize().unwrap();

    let mut repository = None;
    for directory in app.target_directory.ancestors() {
        if let Ok(repo) = gix::open(directory) {
            repository = Some(repo);
        }
    }
    let repository = repository.expect(&format!(
        "did not find a valid git repo in {:?}",
        app.target_directory
    ));

    let latest_release_oid = find_latest_release(&repository).unwrap();
    let new_version = calculate_new_version(&repository, latest_release_oid);
    println!("{}", new_version);
}
