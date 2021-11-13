use crate::data::AtIndicesData;

pub struct AtIndicesIter<'a, T>(AtIndicesData<'a, T>);

impl<'a, T> From<AtIndicesData<'a, T>> for AtIndicesIter<'a, T>
{
    fn from(d: AtIndicesData<'a, T>) -> Self {
        Self(d)
    }
}

impl<'a, T> Iterator for AtIndicesIter<'a, &'a [T]>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for AtIndicesIter<'a, &'a [T]>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T> ExactSizeIterator for AtIndicesIter<'a, &'a [T]> {}