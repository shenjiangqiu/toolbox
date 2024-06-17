use toolbox::TorrentInfo;
fn main() {
    let info: Vec<TorrentInfo> =
        bincode::deserialize_from(std::fs::File::open("info.bin").unwrap()).unwrap();
    let dest = std::path::PathBuf::from("/home/sjq/Downloads/qbt/");
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
        std::fs::rename(from, to).unwrap();
    }
}
