use std::path::PathBuf;

use clap::Parser;
use eyre::{eyre, Ok};
#[derive(Parser, Debug)]
struct Cli {
    /// Source files
    #[clap(required = true)]
    src: Vec<PathBuf>,
    /// Destination directory or file
    dest: PathBuf,

    /// use hard links instead of copying
    #[clap(short = 'H', long)]
    hardlink: bool,
}
struct Entry {
    path: PathBuf,
    name: String,
}
trait FileOp {
    fn op(src: &PathBuf, dest: &PathBuf) -> eyre::Result<()>;
}
struct Hardlink;
struct Copy;
impl FileOp for Hardlink {
    fn op(src: &PathBuf, dest: &PathBuf) -> eyre::Result<()> {
        std::fs::hard_link(src, dest)?;
        Ok(())
    }
}
impl FileOp for Copy {
    fn op(src: &PathBuf, dest: &PathBuf) -> eyre::Result<()> {
        std::fs::copy(src, dest)?;
        Ok(())
    }
}

fn recursive_op<F: FileOp>(src: &PathBuf, dest: &PathBuf) -> eyre::Result<()> {
    if src.is_dir() {
        // dest must be a directory
        std::fs::create_dir_all(dest)?;
        // read the directory and recursively op
        let entries = std::fs::read_dir(src)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let dest: PathBuf = dest.join(name);
            recursive_op::<F>(&path, &dest)?;
        }
    } else {
        let dest_parent = dest.parent().unwrap();
        std::fs::create_dir_all(dest_parent)?;
        F::op(src, dest)?;
    }
    Ok(())
}

fn hardlink(src: &PathBuf, dest: &PathBuf) -> eyre::Result<()> {
    recursive_op::<Hardlink>(src, dest)
}

fn copy(src: &PathBuf, dest: &PathBuf) -> eyre::Result<()> {
    recursive_op::<Copy>(src, dest)
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    // println!("{:?}", cli);
    let first_entry = cli.src.first().unwrap().clone();
    let srcs = cli.src.into_iter().map(|src| {
        // get the file name
        let name = match src.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                let full = src.canonicalize().unwrap();
                full.to_string_lossy().to_string()
            }
        };
        Entry { path: src, name }
    });

    let dest = cli.dest;
    if dest.is_file() && srcs.len() > 1 {
        Err(eyre!("Multiple sources can't be copied to a single file"))?;
    }
    if srcs.len() == 1 && first_entry.is_dir() && dest.is_file() {
        Err(eyre!("Can't copy a directory to a file"))?;
    }

    // policy:
    // - if there are multiple sources, dest must be a directory
    // - if there is a single source, dest can be a file or a directory
    //  - if source is file, dest is file, just copy,
    //  - if source is file, dest is directory, copy to directory
    //  - if source is directory, dest is file, error
    //  - if source is directory, dest is directory, copy to directory
    let dest_is_dir = if srcs.len() > 1 { true } else { dest.is_dir() };

    for src in srcs {
        if cli.hardlink {
            if dest_is_dir {
                let dest = dest.join(&src.name);
                hardlink(&src.path, &dest)?;
            } else {
                hardlink(&src.path, &dest)?;
            }
        } else {
            if dest_is_dir {
                let dest = dest.join(&src.name);
                copy(&src.path, &dest)?;
            } else {
                copy(&src.path, &dest)?;
            }
        }
    }

    Ok(())
}
