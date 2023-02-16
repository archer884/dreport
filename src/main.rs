use std::{process, io, fmt};

use clap::Parser;
use fmtsize::{FmtSize, Conventional};

#[derive(Debug, Parser)]
struct Args {
    root: String,
}

struct PathInfo {
    name: String,
    size: u64,
}

impl fmt::Display for PathInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let size = self.size.fmt_size(Conventional);
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

fn walk(root: &str) -> io::Result<Vec<PathInfo>> {
    todo!()
}
