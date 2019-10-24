#![deny(warnings)]

mod argparse;
mod muxport;

use futures_util::future::select_all;


#[derive(Debug, derive_more::From)]
pub enum Error {
    Io(std::io::Error),
}

type Res<T> = Result<T, Error>;


#[tokio::main]
async fn main() -> Res<()> {
    use argparse::build_commands;
    use muxport::MuxPort;
    use std::env::args;

    let commands = build_commands(args());
    let mps = MuxPort::launch_commands(commands)?;

    println!("{:?}", select_all(mps).await);

    Ok(())
}
