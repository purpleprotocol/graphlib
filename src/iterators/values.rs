// Copyright 2019 Octavian Oncescu

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// Generic values Iterator.
pub struct ValuesIter<'a, T>(pub(crate) Box<dyn 'a + Iterator<Item = &'a T>>);

impl<'a, T> Iterator for ValuesIter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
