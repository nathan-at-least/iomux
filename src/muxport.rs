use crate::Res;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::process::ExitStatus;
use tokio::prelude::Future;
use tokio::net::process::{Child, Command};


#[derive(Debug)]
pub struct MuxPort {
    child: Child,
}


impl MuxPort {
    pub fn launch_commands(cmds: Vec<Command>) -> Res<Vec<MuxPort>> {
        // Result::collect(cmds.map(|c| c.spawn()))
        let mut mps = vec![];

        for mut cmd in cmds {
            let mp = MuxPort {
                child: cmd.spawn()?,
            };

            mps.push(mp);
        }

        Ok(mps)
    }
}


impl Future for MuxPort {
    type Output = Result<ExitStatus, tokio::io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let unpinself = self.get_mut();
        let childpin = Pin::new(&mut unpinself.child);
        Future::poll(childpin, cx)
    }
}
