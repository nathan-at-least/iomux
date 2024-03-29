use crate::lpq::LinePeekerQueue;
use crate::Res;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::process::ExitStatus;
use tokio::net::process::{Child, Command};
use tokio::stream::Stream;

#[derive(Debug)]
pub struct MuxPort {
    child: Option<Child>,
    outbuf: LinePeekerQueue,
    errbuf: LinePeekerQueue,
}

impl MuxPort {
    pub fn launch_commands(cmds: Vec<Command>) -> Res<Vec<MuxPort>> {
        // Result::collect(cmds.map(|c| c.spawn()))
        let mut mps = vec![];

        for mut cmd in cmds {
            let mp = MuxPort {
                child: Some(cmd.spawn()?),
                outbuf: LinePeekerQueue::new(),
                errbuf: LinePeekerQueue::new(),
            };

            mps.push(mp);
        }

        Ok(mps)
    }
}

#[derive(Debug)]
pub enum MuxStreamItem {
    OutLine(String),
    ErrLine(String),
    Status(ExitStatus),
}

impl Stream for MuxPort {
    type Item = Result<MuxStreamItem, tokio::io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        use tokio::future::Future;
        use tokio::io::AsyncRead;
        use Poll::{Pending, Ready};

        fn poll_read_child_buf<R>(
            optsrc: &mut Option<R>,
            cx: &mut Context,
            lpq: &mut LinePeekerQueue,
        ) -> Poll<Result<String, tokio::io::Error>>
        where
            R: AsyncRead + std::marker::Unpin,
        {
            if let &mut Some(ref mut src) = optsrc {
                match AsyncRead::poll_read_buf(Pin::new(src), cx, lpq) {
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

        if let Some(ref mut child) = unpinself.child {
            match poll_read_child_buf(child.stdout(), cx, &mut unpinself.outbuf) {
                Ready(res) => Ready(Some(res.map(MuxStreamItem::OutLine))),
                Pending => match poll_read_child_buf(child.stderr(), cx, &mut unpinself.errbuf) {
                    Ready(res) => Ready(Some(res.map(MuxStreamItem::ErrLine))),
                    Pending => match Future::poll(Pin::new(child), cx) {
                        Ready(res) => {
                            unpinself.child = None;
                            Ready(Some(res.map(MuxStreamItem::Status)))
                        }
                        Pending => Poll::Pending,
                    },
                },
            }
        } else {
            if let Some(s) = unpinself.outbuf.peek_drop_line_or_end() {
                Ready(Some(Ok(MuxStreamItem::OutLine(String::from(s)))))
            } else if let Some(s) = unpinself.errbuf.peek_drop_line_or_end() {
                Ready(Some(Ok(MuxStreamItem::ErrLine(String::from(s)))))
            } else {
                Ready(None)
            }
        }
    }
}
