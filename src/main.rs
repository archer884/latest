use std::fs::{self, DirEntry};
use std::{env, io};
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

fn list_files(files: impl IntoIterator<Item = DirEntry>) {
    files
        .into_iter()
        .for_each(|x| println!("{}", x.path().display()));
}

fn read_files() -> io::Result<impl IntoIterator<Item = DirEntry>> {
    let mut entries: Vec<_> = fs::read_dir(env::current_dir()?)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let metadata = entry.metadata().ok()?;
            Some((metadata, entry))
        })
        .filter(|x| x.0.file_type().is_file())
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
