use crate::Res;
use crate::lpq::LinePeekerQueue;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::process::ExitStatus;
use tokio::stream::Stream;
use tokio::net::process::{Child, Command};


#[derive(Debug)]
pub struct MuxPort {
    child: Child,
    outbuf: LinePeekerQueue,
    errbuf: LinePeekerQueue,
}


impl MuxPort {
    pub fn launch_commands(cmds: Vec<Command>) -> Res<Vec<MuxPort>> {
        // Result::collect(cmds.map(|c| c.spawn()))
        let mut mps = vec![];

        for mut cmd in cmds {
            let mp = MuxPort {
                child: cmd.spawn()?,
                outbuf: LinePeekerQueue::new(),
                errbuf: LinePeekerQueue::new(),
            };

            mps.push(mp);
        }

        Ok(mps)
    }
}


pub enum MuxStreamItem {
    OutLine(String),
    ErrLine(String),
    Status(ExitStatus),
}


impl Stream for MuxPort {
    type Item = Result<MuxStreamItem, tokio::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        use tokio::future::Future;
        use tokio::io::AsyncRead;
        use Poll::{Ready, Pending};

        fn poll_read_child_buf<R>(optsrc: &mut Option<R>, cx: &mut Context, lpq: &mut LinePeekerQueue) -> Poll<Result<String, tokio::io::Error>>
            where R: AsyncRead + std::marker::Unpin,
        {
            if let Some(src) = *optsrc {
                match AsyncRead::poll_read_buf(Pin::new(&mut src), cx, &mut lpq) {
                    Pending => Pending,
                    Ready(Err(e)) => Ready(Err(e)),
                    Ready(Ok(_bytecount)) => {
                        if let Some(line) = lpq.peek_drop_line() {
                            Ready(Ok(line.to_string()))
                        } else {
                            Pending
                        }
                    }
                }
            } else {
                Pending
            }
        }

        let unpinself = self.get_mut();

        match poll_read_child_buf(unpinself.child.stdout(), cx, &mut self.outbuf) {
            Ready(res) => Ready(Some(res.map(MuxStreamItem::OutLine))),
            Pending => match poll_read_child_buf(unpinself.child.stderr(), cx, &mut self.errbuf) {
                Ready(res) => Ready(Some(res.map(MuxStreamItem::ErrLine))),
                Pending => match Future::poll(Pin::new(&mut unpinself.child), cx) {
                    Ready(res) => Ready(Some(res.map(MuxStreamItem::Status))),
                    Pending => Poll::Pending,
                }
            }
        }
    }
}
