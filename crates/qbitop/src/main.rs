use std::{
    fs::File,
    io::{BufReader, BufWriter},
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};

use clap::{Args, Parser};
use qbit_rs::{
    model::{AddTorrentArg, Credential, TorrentFile},
    Qbit,
};
use tokio::sync::Mutex;
use toolbox::TorrentInfo;

#[derive(Args, Debug)]
struct QbitArgs {
    #[clap(short, long)]
    username: String,
    #[clap(short, long)]
    password: String,
    #[clap(short = 'U', long)]
    url: String,
    #[clap(short, long, default_value = "info.bin")]
    infofile: PathBuf,
}
#[derive(Args, Debug)]
struct InfoArgs {
    #[clap(short, long, default_value = "info.bin")]
    infofile: PathBuf,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    operation: Operation,
}
#[derive(clap::Subcommand, Debug)]
enum Operation {
    DumpInfo(QbitArgs),
    AddTorrents(QbitArgs),
    ShowInfo(InfoArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.operation {
        Operation::DumpInfo(QbitArgs {
            username,
            password,
            url,
            infofile,
        }) => {
            let qbit = Qbit::new(url.as_str(), Credential::new(&username, &password));
            let qbit = Arc::new(qbit);
            let all_list = qbit.get_torrent_list(Default::default()).await.unwrap();
            let infos = Arc::new(Mutex::new(Vec::new()));
            let mut handles = Vec::new();
            for t in all_list {
                let qbit = qbit.clone();
                let infos = infos.clone();
                let h = tokio::spawn(async move {
                    let torrent = qbit.export_torrent(t.hash.as_ref().unwrap()).await.unwrap();
                    let torrent = torrent.to_vec();
                    infos.lock().await.push(TorrentInfo { info: t, torrent });
                });
                handles.push(h);
            }
            for h in handles {
                h.await.unwrap();
            }
            let writer = BufWriter::new(File::create(infofile).unwrap());
            bincode::serialize_into(writer, infos.lock().await.deref()).unwrap();
        }
        Operation::AddTorrents(QbitArgs {
            username,
            password,
            url,
            infofile,
        }) => {
            let qbit = Qbit::new(url.as_str(), Credential::new(&username, &password));
            let info: Vec<TorrentInfo> =
                bincode::deserialize_from(File::open(infofile).unwrap()).unwrap();
            for i in info {
                qbit.add_torrent(AddTorrentArg {
                    source: qbit_rs::model::TorrentSource::TorrentFiles {
                        torrents: vec![TorrentFile {
                            filename: i.info.name.clone().unwrap(),
                            data: i.torrent,
                        }],
                    },
                    category: i.info.category,
                    skip_checking: Some("true".to_string()),
                    paused: Some("true".to_string()),
                    auto_torrent_management: Some(true),
                    ..Default::default()
                })
                .await
                .unwrap();
            }
        }
        Operation::ShowInfo(InfoArgs { infofile }) => {
            println!("{:?}", infofile);
            let info: Vec<TorrentInfo> =
                bincode::deserialize_from(BufReader::new(File::open(infofile).unwrap())).unwrap();
            for i in info {
                println!("{:?}", i.info.save_path);
                println!("{:?}", i.info.content_path);
                println!("{:?}", i.info.name);
            }
        }
    }
}
