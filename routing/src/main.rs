mod communication;

use crate::communication::Connection;
use env_logger::{Builder, Target};
use log::LevelFilter;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    Ok(())
}
