use memmem::{Searcher, TwoWaySearcher};

pub struct UnOwnedLine<'a> {
    buf: &'a [u8],
    finished: bool,
    phrase_len: usize,
    searcher: TwoWaySearcher<'a>,
}

impl<'a> Iterator for UnOwnedLine<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.searcher.search_in(self.buf) {
            Some(pos) => {
                let chunk = &self.buf[..pos];
                self.buf = &self.buf[pos + self.phrase_len..];

                Some(chunk)
            }
            None if !self.buf.is_empty() && !self.finished => {
                self.finished = true;
                Some(self.buf)
            }
            None => None,
        };

        result.and_then(|result| {
            if !result.is_empty() && result != "\n".as_bytes() {
                Some(result)
            } else {
                None
            }
        })
    }
}

impl<'a> UnOwnedLine<'a> {
    pub fn search(phrase: &'a [u8], buf: &'a [u8]) -> Self {
        Self {
            buf,
            finished: false,
            phrase_len: phrase.len(),
            searcher: TwoWaySearcher::new(phrase),
        }
    }

    pub fn double_line(buf: &'a [u8]) -> Self {
        Self::search("\n\n".as_bytes(), buf)
    }

    pub fn single_line(buf: &'a [u8]) -> Self {
        Self::search("\n".as_bytes(), buf)
    }
}
