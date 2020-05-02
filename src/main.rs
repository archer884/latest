use chrono::{DateTime, Local};
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
/// List recent files.
struct Options {
    /// A path to list.
    path: Option<String>,
    /// The number of files to list.
    #[structopt(short, long)]
    count: Option<usize>,
    /// Show only files from today.
    #[structopt(short, long)]
    today: bool,
}

fn main() -> io::Result<()> {
    let options = Options::from_args();
    let files = read_files(options.path, options.today)?;

    match options.count {
        None => list_files(files),
        Some(count) => list_files(files.into_iter().take(count)),
    }

    Ok(())
}

fn list_files(files: impl IntoIterator<Item = PathBuf>) {
    files.into_iter().for_each(|x| println!("{}", x.display()));
}

fn read_files(path: Option<String>, today: bool) -> io::Result<impl IntoIterator<Item = PathBuf>> {
    let path: PathBuf = path
        .map(|path| Ok(path.into()))
        .unwrap_or_else(env::current_dir)?;

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?.path();
            let metadata = entry.metadata().ok()?;
            let modified = metadata
                .modified()
                .expect("Critical error: last write time unavailable");

            if !metadata.file_type().is_file()
                || is_hidden(&entry)
                || (today && !is_from_today(modified))
            {
                return None;
            }

            Some((modified, entry))
        })
        .collect();

    entries.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(entries.into_iter().map(|(_, x)| x))
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|x| x.to_str().map(|x| x.starts_with('.')))
        .unwrap_or(true)
}

fn is_from_today(modified: impl Into<DateTime<Local>>) -> bool {
    Local::today() == modified.into().date()
}
