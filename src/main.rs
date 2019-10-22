#![deny(warnings)]

mod argparse;
mod muxport;


#[derive(Debug, derive_more::From)]
enum Error {
    Io(std::io::Error),
    Tokio(tokio::io::Error),
}

type Res<T> = Result<T, Error>;


fn main() -> std::io::Result<()> {
    use argparse::build_commands;
    use muxport::MuxPort;
    use std::env::args;

    let commands = build_commands(args());
    let mps = MuxPort::launch_commands(commands)?;

    println!("{:?}", mps);

    Ok(())
}
