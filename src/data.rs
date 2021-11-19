pub(crate) struct SelectIndicesBase<'a, T, I>
{
    pub(crate) data: T,
    pub(crate) indices: &'a [I],
    pub(crate) start: usize,
    pub(crate) end: usize,
}

use num_traits::{ PrimInt, ToPrimitive };

use std::collections::HashSet;

impl<T, I: Copy + Clone + PrimInt + ToPrimitive> SelectIndicesBase<'_, &[T], I>
{
    pub(crate) fn safety_check(slice: &[T], indices: &[I])
    {
        let len = slice.len();
        let indices_len = indices.len();

        // If indices is longer than the slice, either there are
        // duplicates, or some indices are out of bounds.
        assert!(indices_len <= len); 

        let mut indexset = HashSet::with_capacity(indices_len);
        // TODO: Safety checks without heap allocation
        
        indices.iter().for_each(|&i| {
            let i = i.to_usize().unwrap();
            assert!(i < len);
            assert!(indexset.insert(i));
        });
    }
}

impl<T, I: Copy + Clone + PrimInt + ToPrimitive> SelectIndicesBase<'_, &mut [T], I>
{
    pub(crate) fn safety_check(slice: &mut [T], indices: &[I])
    {
        let len = slice.len();
        let indices_len = indices.len();

        // If indices is longer than the slice, either there are
        // duplicates, or some indices are out of bounds.
        assert!(indices_len <= len); 

        let mut indexset = HashSet::with_capacity(indices_len);
        // TODO: Safety checks without heap allocation
        
        indices.iter().for_each(|&i| {
            let i = i.to_usize().unwrap();
            assert!(i < len);
            assert!(indexset.insert(i));
        });
    }
}

impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> Iterator for SelectIndicesBase<'a, &'a [T], I>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            let ind = unsafe { *self.indices.get_unchecked(self.start) };
            self.start += 1;
            return Some(unsafe { self.data.get_unchecked(ind.to_usize().unwrap()) });
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
impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> DoubleEndedIterator for SelectIndicesBase<'a, &'a [T], I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            self.end -= 1;
            let ind = unsafe { * self.indices.get_unchecked(self.end) };
            return Some(unsafe { self.data.get_unchecked(ind.to_usize().unwrap()) });
        }
        else {
            return None;
        }
    }
}
impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> ExactSizeIterator for SelectIndicesBase<'a, &'a [T], I> {}

impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> Iterator for SelectIndicesBase<'a, &'a mut [T], I>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            let ind = unsafe { *self.indices.get_unchecked(self.start) };
            self.start += 1;
            return Some(unsafe { &mut *self.data.as_mut_ptr().add(ind.to_usize().unwrap()) });
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
impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> DoubleEndedIterator for SelectIndicesBase<'a, &'a mut [T], I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end
        {
            self.end -= 1;
            let ind = unsafe { *self.indices.get_unchecked(self.end) };
            return Some(unsafe { &mut *self.data.as_mut_ptr().add(ind.to_usize().unwrap()) });
        }
        else {
            return None;
        }
    }
}
impl<'a, T: 'a, I: Copy + Clone + PrimInt + ToPrimitive> ExactSizeIterator for SelectIndicesBase<'a, &'a mut [T], I> {}