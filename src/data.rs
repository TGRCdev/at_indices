//
pub(crate) struct SelectIndicesBase<'a, T>
{
    pub(crate) data: T,
    pub(crate) indices: &'a [usize],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl<'a, T: 'a> Iterator for SelectIndicesBase<'a, &'a [T]>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            let ind = unsafe { *self.indices.get_unchecked(self.start) };
            self.start += 1;
            return Some(unsafe { self.data.get_unchecked(ind) });
        }
        else {
            return None;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.indices.len();
        return (len, Some(len));
    }
}
impl<'a, T: 'a> DoubleEndedIterator for SelectIndicesBase<'a, &'a [T]>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            self.end -= 1;
            let ind = unsafe { * self.indices.get_unchecked(self.end) };
            return Some(unsafe { self.data.get_unchecked(ind) });
        }
        else {
            return None;
        }
    }
}
impl<'a, T: 'a> ExactSizeIterator for SelectIndicesBase<'a, &'a [T]> {}

impl<'a, T: 'a> Iterator for SelectIndicesBase<'a, &'a mut [T]>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            let ind = unsafe { *self.indices.get_unchecked(self.start) };
            self.start += 1;
            return Some(unsafe { &mut *self.data.as_mut_ptr().add(ind) });
        }
        else {
            return None;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.indices.len();
        return (len, Some(len));
    }
}
impl<'a, T: 'a> DoubleEndedIterator for SelectIndicesBase<'a, &'a mut [T]>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            self.end -= 1;
            let ind = unsafe { *self.indices.get_unchecked(self.end) };
            return Some(unsafe { &mut *self.data.as_mut_ptr().add(ind) });
        }
        else {
            return None;
        }
    }
}
impl<'a, T: 'a> ExactSizeIterator for SelectIndicesBase<'a, &'a mut [T]> {}