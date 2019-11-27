#![deny(warnings)]

use std::io::{stderr, stdout, Write};

struct OE {
    isout: bool,
}

impl OE {
    fn new() -> OE {
        OE { isout: true }
    }

    fn flip(&mut self) {
        self.isout = !self.isout;
    }
}

impl Write for OE {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.isout {
            stdout().write(buf)
        } else {
            stderr().write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.isout {
            stdout().flush()
        } else {
            stderr().flush()
        }
    }
}

#[derive(Debug, derive_more::From)]
enum MyError {
    Std(std::io::Error),
    Parse(std::num::ParseIntError),
}

fn main() -> Result<(), MyError> {
    let mut oe = OE::new();

    for arg in std::env::args().skip(1) {
        let count: i32 = arg.parse()?;
        for i in 1..count {
            write!(&mut oe, "{}, ", i)?;
        }
        write!(&mut oe, "{}\n", count)?;

        oe.flush()?;
        oe.flip();
    }

    Ok(())
}
