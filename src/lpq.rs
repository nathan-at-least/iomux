use bytes::BufMut;


#[derive(Debug)]
pub struct LinePeekerQueue {
    buf: String,
    dropix: Option<usize>,
}

impl LinePeekerQueue {
    pub fn new() -> LinePeekerQueue {
        LinePeekerQueue {
            buf: String::new(),
            dropix: None,
        }
    }

    pub fn peek_drop_line(&mut self) -> Option<&str> {
        if let Some(ix) = self.dropix {
            self.buf.replace_range(..ix+1, "");
        }

        self.dropix = self.buf.find('\n');
        self.dropix.map(move |ix| &self.buf[..ix+1])
    }
}

impl BufMut for LinePeekerQueue {
    fn remaining_mut(&self) -> usize {
        std::usize::MAX - self.buf.as_bytes().len()
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.buf.as_mut_vec().advance_mut(cnt)
    }

    unsafe fn bytes_mut(&mut self) -> &mut [u8] {
        self.buf.as_mut_vec().bytes_mut()
    }
}


#[cfg(test)]
mod tests {
    use super::LinePeekerQueue;

    impl From<&str> for LinePeekerQueue {
        fn from(s: &str) -> LinePeekerQueue {
            LinePeekerQueue {
                buf: s.into(),
                dropix: None,
            }
        }
    }

    #[test]
    fn peek_drop_two_lines() {
        for &vector in &["foo\nbarf\n", "foo\nbarf\nquxum"] {
            let mut lpq = LinePeekerQueue::from(vector);

            assert_eq!(Some("foo\n"), lpq.peek_drop_line());
            assert_eq!(Some("barf\n"), lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line());
        }
    }
}
