use crate::data::AtIndicesData;

pub struct AtIndicesIter<'a, T>(AtIndicesData<'a, T>);

impl<'a, T> From<AtIndicesData<'a, T>> for AtIndicesIter<'a, T>
{
    fn from(d: AtIndicesData<'a, T>) -> Self {
        Self(d)
    }
}