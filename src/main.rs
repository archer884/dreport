use std::{fmt, fs, io, path::PathBuf, process};

use clap::Parser;
use fmtsize::{Conventional, FmtSize};

#[derive(Debug, Parser)]
struct Args {
    root: String,
}

struct DirInfo {
    path: PathBuf,
    len: u64,
}

impl DirInfo {
    fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            len: 0,
        }
    }
}

impl fmt::Display for DirInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.path.display();
        let size = self.len.fmt_size(Conventional);
        write!(f, "{name}\t{size}")
    }
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let paths = walk(&args.root)?;

    for info in paths {
        println!("{info}");
    }

    Ok(())
}

fn walk(root: &str) -> io::Result<Vec<DirInfo>> {
    let mut paths = get_directories(root)?;
    paths.iter_mut().for_each(populate_dir_size);
    Ok(paths)
}

fn populate_dir_size(info: &mut DirInfo) {
    let len: u64 = walkdir::WalkDir::new(&info.path)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let meta = entry.metadata().ok()?;
            meta.file_type().is_file().then(|| meta.len())
        })
        .sum();
    info.len = len;
}

fn get_directories(root: &str) -> io::Result<Vec<DirInfo>> {
    fs::read_dir(root)?
        .map(|entry| entry.and_then(|entry| entry.file_type().map(|t| (t, entry.path()))))
        .filter_map(|entry| {
            entry
                .map(|(t, path)| t.is_dir().then_some(path))
                .transpose()
        })
        .map(|path| path.map(DirInfo::new))
        .collect()
}
