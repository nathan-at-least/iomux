use crate::Res;
use core::pin::Pin;
use std::process::ExitStatus;
use tokio::net::process::{Child, Command};
use futures_util::future::Future;
use futures_util::task::{Context, Poll};


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
        self.child.poll(cx)
    }
}
