use qbit_rs::model::Torrent;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct TorrentInfo {
    pub info: Torrent,
    pub torrent: Vec<u8>,
}


pub fn hello() {
    println!("Hello, world!");
}
