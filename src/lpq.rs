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
        self.peek_drop(false)
    }

    pub fn peek_drop_line_or_end(&mut self) -> Option<&str> {
        self.peek_drop(true)
    }

    fn peek_drop(&mut self, end: bool) -> Option<&str> {
        println!("DEBUG {:?}.peek_drop({:?})...", &self, end);
        if let Some(ix) = self.dropix {
            self.buf.replace_range(..ix, "");
        }

        println!("DEBUG {:?}.peek_drop({:?}) after replace.", &self, end);

        self.dropix = self.buf.find('\n').map(|ix| ix + 1);
        println!(
            "DEBUG {:?}.peek_drop({:?}) after dropix update.",
            &self, end
        );
        if end {
            self.dropix = self.dropix.or(Some(self.buf.len()));
            println!(
                "DEBUG {:?}.peek_drop({:?}) after end=true update",
                &self, end
            );
        }
        let result = self.dropix.map(move |ix| &self.buf[..ix]);
        println!("DEBUG returning -> {:?}", &result);
        result
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
    macro_rules! tests_for_vector {
        ( $vecname:ident, $vecval:expr, [ $( $testname:ident : $testexpr:expr ),* ] ) => {
            mod $vecname {
                use super::super::LinePeekerQueue;

                $(
                    #[test]
                    fn $testname() {
                        ($testexpr)(LinePeekerQueue {
                            buf: String::from($vecval),
                            dropix: None,
                        })
                    }
                )*
            }
        }
    }

    tests_for_vector!(empty, "", [
        peek_drop_line:
        |mut lpq: LinePeekerQueue| {
            assert_eq!(None, lpq.peek_drop_line());
        },

        peek_drop_line_idempotent:
        |mut lpq: LinePeekerQueue| {
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line());
        },

        peek_drop_line_or_end_idempotent:
        |mut lpq: LinePeekerQueue| {
            assert_eq!(None, lpq.peek_drop_line_or_end());
            assert_eq!(None, lpq.peek_drop_line_or_end());
        },

        peek_drop_line_then_peek_drop_line_or_end:
        |mut lpq: LinePeekerQueue| {
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line_or_end());
        },

        peek_drop_line_or_end_then_peek_drop_line:
        |mut lpq: LinePeekerQueue| {
            assert_eq!(None, lpq.peek_drop_line_or_end());
            assert_eq!(None, lpq.peek_drop_line());
        }
    ]);
}

#[cfg(test)]
mod tests_old {
    use super::LinePeekerQueue;

    fn make(s: &str) -> LinePeekerQueue {
        LinePeekerQueue {
            buf: s.into(),
            dropix: None,
        }
    }

    mod empty {
        use super::make;

        #[test]
        fn peek_drop_line() {
            let mut lpq = make("");
            assert_eq!(None, lpq.peek_drop_line());
        }

        #[test]
        fn peek_drop_line_idempotent() {
            let mut lpq = make("");
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line());
        }

        #[test]
        fn peek_drop_line_or_end_idempotent() {
            let mut lpq = make("");
            assert_eq!(None, lpq.peek_drop_line_or_end());
            assert_eq!(None, lpq.peek_drop_line_or_end());
        }

        #[test]
        fn peek_drop_line_then_peek_drop_line_or_end() {
            let mut lpq = make("");
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line_or_end());
        }

        #[test]
        fn peek_drop_line_or_end_then_peek_drop_line() {
            let mut lpq = make("");
            assert_eq!(None, lpq.peek_drop_line_or_end());
            assert_eq!(None, lpq.peek_drop_line());
        }
    }

    #[test]
    fn peek_drop_two_lines() {
        for &vector in &["foo\nbarf\n", "foo\nbarf\nquxum"] {
            let mut lpq = make(vector);

            assert_eq!(Some("foo\n"), lpq.peek_drop_line());
            assert_eq!(Some("barf\n"), lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(None, lpq.peek_drop_line());
            assert_eq!(Some("quxum"), lpq.peek_drop_line_or_end());
            assert_eq!(None, lpq.peek_drop_line_or_end());
        }
    }
}
