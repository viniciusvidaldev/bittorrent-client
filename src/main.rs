mod torrent;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::torrent::{Keys, Torrent};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Info { torrent_path: PathBuf },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Info { torrent_path } => {
            let torrent = Torrent::read(torrent_path).await?;
            println!("Tracker URL: {}", torrent.announce);
            if let Keys::SingleFile { length } = torrent.info.keys {
                println!("Length: {length}");
            }

            let hash = torrent.info_hash();
            println!("{}", hex::encode(hash));

            for piece_hash in torrent.info.pieces.iter() {
                println!("Piece hash: {}", hex::encode(piece_hash));
            }
        }
    }

    Ok(())
}
