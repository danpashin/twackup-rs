use memmem::{Searcher, TwoWaySearcher};

#[derive(Debug)]
pub(crate) struct UnOwnedLine<'buf> {
    buf: &'buf [u8],
    finished: bool,
    phrase_len: usize,
    searcher: TwoWaySearcher<'buf>,
}

impl<'buf> Iterator for UnOwnedLine<'buf> {
    type Item = &'buf [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.searcher.search_in(self.buf) {
            Some(pos) => {
                let chunk = self.buf.get(..pos)?;
                self.buf = self.buf.get(pos + self.phrase_len..)?;

                Some(chunk)
            }
            None if !self.buf.is_empty() && !self.finished => {
                self.finished = true;
                Some(self.buf)
            }
            None => None,
        };

        result.and_then(|result| (!result.is_empty() && result != b"\n").then_some(result))
    }
}

impl<'buf> UnOwnedLine<'buf> {
    #[inline]
    #[must_use]
    pub(crate) fn search(phrase: &'buf [u8], buf: &'buf [u8]) -> Self {
        Self {
            buf,
            finished: false,
            phrase_len: phrase.len(),
            searcher: TwoWaySearcher::new(phrase),
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn double_line(buf: &'buf [u8]) -> Self {
        Self::search(b"\n\n", buf)
    }

    #[inline]
    #[must_use]
    pub(crate) fn single_line(buf: &'buf [u8]) -> Self {
        Self::search(b"\n", buf)
    }
}
