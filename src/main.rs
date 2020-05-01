use std::path::{Path, PathBuf};
use std::{env, fs, io};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
/// List files in descending order of last write.
struct Options {
    /// The number of files to list.
    count: Option<usize>,
}

fn main() -> io::Result<()> {
    let options = Options::from_args();
    let files = read_files();

    match options.count {
        None => list_files(files?),
        Some(count) => list_files(files?.into_iter().take(count)),
    }

    Ok(())
}

fn list_files(files: impl IntoIterator<Item = PathBuf>) {
    files.into_iter().for_each(|x| println!("{}", x.display()));
}

fn read_files() -> io::Result<impl IntoIterator<Item = PathBuf>> {
    let mut entries: Vec<_> = fs::read_dir(env::current_dir()?)?
        .filter_map(|entry| {
            let entry = entry.ok()?.path();
            let metadata = entry.metadata().ok()?;
            Some((metadata, entry))
        })
        .filter(|x| x.0.file_type().is_file() && !is_hidden(&x.1))
        .map(|(meta, entry)| {
            (
                meta.modified()
                    .expect("Critical error: last write time unavailable"),
                entry,
            )
        })
        .collect();

    entries.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(entries.into_iter().map(|(_, x)| x))
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|x| x.to_str().map(|x| x.starts_with(".")))
        .unwrap_or(true)
}
