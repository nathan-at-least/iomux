#![deny(warnings)]

mod argparse;
mod lpq;
mod muxport;

// use futures::select_all;

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
    assert_eq!(
        commands.len(),
        1,
        "only single command supported right now..."
    );

    let mut mps = MuxPort::launch_commands(commands)?;
    assert_eq!(mps.len(), 1, "only single command supported right now...");

    {
        use tokio::prelude::*;

        let mp = &mut mps[0];

        while let Some(msitem) = mp.next().await {
            println!("{:?}", msitem);
        }
    }

    Ok(())
}
