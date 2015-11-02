use std::iter::Peekable;

pub struct ConsumeWhile<'a, I: Iterator + 'a, F> where I::Item: 'a {
    inner: &'a mut Peekable<I>,
    func: F
}

impl<'a, I: Iterator + 'a, F: FnMut(&I::Item) -> bool> Iterator for ConsumeWhile<'a, I, F> where I::Item: 'a {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let accepted = self.inner.peek().map(&mut self.func).unwrap_or(false);

        if accepted {
            self.inner.next()
        } else {
            None
        }
    }
}

pub trait ConsumeWhileExt<I: Iterator> {
    fn consume_while<F: FnMut(&I::Item) -> bool>(&mut self, func: F) -> ConsumeWhile<I, F>;
}

impl<I: Iterator> ConsumeWhileExt<I> for Peekable<I> {
    fn consume_while<F: FnMut(&I::Item) -> bool>(&mut self, func: F) -> ConsumeWhile<I, F> {
        ConsumeWhile {
            inner: self,
            func: func
        }
    }
}
