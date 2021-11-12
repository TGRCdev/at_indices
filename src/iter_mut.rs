use crate::AtIndicesData;

pub struct AtIndicesIterMut<'a, T>(AtIndicesData<'a, T>);

impl<'a, T> From<AtIndicesData<'a, T>> for AtIndicesIterMut<'a, T>
{
    fn from(d: AtIndicesData<'a, T>) -> Self {
        Self(d)
    }
}

impl<'a, T> Iterator for AtIndicesIterMut<'a, &'a mut [T]>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, T> DoubleEndedIterator for AtIndicesIterMut<'a, &'a mut [T]>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<'a, T> ExactSizeIterator for AtIndicesIterMut<'a, &'a mut [T]> {}