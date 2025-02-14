use clap::Subcommand;
use std::path::PathBuf;

pub mod op;
pub mod sdb;

#[derive(Debug, Subcommand)]
pub enum Command {
  List,
  Dump {
    package: String,

    #[clap(short, long)]
    out: Option<PathBuf>,
  },
  DumpAll {
    #[clap(short, long)]
    out: Option<PathBuf>,
    // TODO skip unwanted packages
  },
  Upload {
    package: Option<String>,

    #[clap(long)]
    server: String,
  },
}

pub fn main() {
  tracing_subscriber::fmt::init();
  tracing::trace!("init");

  let sdb_path = sdb::sdb_path();
  println!("SDB path: {:?}", sdb_path);
}
