use std::{io::BufReader, path::PathBuf};

use clap::Parser;
use toolbox::TorrentInfo;
#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    dry_run: bool,
}
fn main() {
    let cli = Cli::parse();
    let info: Vec<TorrentInfo> =
        bincode::deserialize_from(BufReader::new(std::fs::File::open("info.bin").unwrap()))
            .unwrap();
    let dest = std::path::PathBuf::from("/home/sjq/usb/Downloads/qbt/");
    for t in info {
        let category = t.info.category;
        let from = std::path::PathBuf::from(t.info.content_path.unwrap());
        let from = if from.starts_with("/home/sjq") {
            if from.starts_with("/home/sjq/sjqbcachefs") {
                let mut from = from.to_string_lossy().to_string();
                from.replace_range(..21, "/home/sjq/usb/sjqbcachefs");
                PathBuf::from(from)
            } else {
                assert!(from.starts_with("/home/sjq/usb"));
                from
            }
        } else {
            panic!("save_path is not or /home/sjq");
        };
        let to = match category {
            Some(c) => {
                let dest = dest.join(c);
                dest.join(t.info.name.unwrap())
            }
            None => dest.join(t.info.name.unwrap()),
        };
        if cli.dry_run {
            println!("from: {:?}, to: {:?}", from, to);
        } else {
            let to_parent = to.parent().unwrap();
            if !to_parent.exists() {
                std::fs::create_dir_all(to_parent).unwrap();
            }
            std::fs::rename(&from, &to).unwrap_or_else(|e| println!("error: {e} {:?},{:?}",from,to));
        }
    }
}
