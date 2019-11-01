#[derive(Debug)]
pub struct LinePeekerQueue {
    buf: String,
    dropix: Option<usize>,
}

impl LinePeekerQueue {
    pub fn peek_drop_line(&mut self) -> Option<&str> {
        if let Some(ix) = self.dropix {
            self.buf.replace_range(..ix+1, "");
        }

        self.dropix = self.buf.find('\n');
        self.dropix.map(move |ix| &self.buf[..ix+1])
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
