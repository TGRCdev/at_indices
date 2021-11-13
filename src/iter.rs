use crate::data::SelectIndicesBase;

pub struct SelectIndicesIter<'a, T>(SelectIndicesBase<'a, T>);

impl<'a, T> From<SelectIndicesBase<'a, T>> for SelectIndicesIter<'a, T>
{
    fn from(d: SelectIndicesBase<'a, T>) -> Self {
        Self(d)
    }
}

impl<'a, T> Iterator for SelectIndicesIter<'a, &'a [T]>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for SelectIndicesIter<'a, &'a [T]>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T> ExactSizeIterator for SelectIndicesIter<'a, &'a [T]> {}