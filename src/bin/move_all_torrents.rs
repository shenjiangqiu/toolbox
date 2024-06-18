use std::io::BufReader;

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
            from
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
            std::fs::rename(from, to).unwrap();
        }
    }
}
