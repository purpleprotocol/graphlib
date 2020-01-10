// Copyright 2019 Octavian Oncescu

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
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
