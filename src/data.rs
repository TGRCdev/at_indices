pub(crate) struct AtIndicesData<'a, T>
{
    pub(crate) data: T,
    pub(crate) indices: &'a [usize],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl<'a, T: 'a> Iterator for AtIndicesData<'a, &'a [T]>
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
}
impl<'a, T: 'a> DoubleEndedIterator for AtIndicesData<'a, &'a [T]>
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
impl<'a, T: 'a> ExactSizeIterator for AtIndicesData<'a, &'a [T]> {}

impl<'a, T: 'a> Iterator for AtIndicesData<'a, &'a mut [T]>
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
}
impl<'a, T: 'a> DoubleEndedIterator for AtIndicesData<'a, &'a mut [T]>
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
impl<'a, T: 'a> ExactSizeIterator for AtIndicesData<'a, &'a mut [T]> {}