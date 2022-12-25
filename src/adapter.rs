use std::iter::Peekable;

pub struct PeekWhile<'a, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    iter: &'a mut Peekable<I>,
    predicate: F,
}

impl<I, F> Iterator for PeekWhile<'_, I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.iter.next_if(&mut self.predicate)
    }
}

pub trait PeekWhileExt<I>: Iterator
where
    I: Iterator,
{
    fn peek_while<F>(&mut self, predicate: F) -> PeekWhile<I, F>
    where
        F: FnMut(&I::Item) -> bool;
}

impl<I: Iterator> PeekWhileExt<I> for Peekable<I> {
    fn peek_while<F>(&mut self, predicate: F) -> PeekWhile<I, F>
    where
        F: FnMut(&I::Item) -> bool,
    {
        PeekWhile {
            iter: self,
            predicate,
        }
    }
}
