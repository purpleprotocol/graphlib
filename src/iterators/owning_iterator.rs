// Copyright 2019 Octavian Oncescu

use crate::vertex_id::VertexId;

#[cfg(feature = "no_std")]
extern crate alloc;

#[cfg(feature = "no_std")]
use alloc::boxed::Box;

#[cfg(not(feature = "no_std"))]
use std::fmt::Debug;

#[cfg(not(feature = "no_std"))]
use std::marker::PhantomData;

#[cfg(feature = "no_std")]
use core::marker::PhantomData;

#[cfg(feature = "no_std")]
use core::mem;

#[cfg(not(feature = "no_std"))]
use std::mem;

#[cfg(not(feature = "no_std"))]
use std::collections::VecDeque;

#[cfg(feature = "no_std")]
use alloc::collections::VecDeque;

/// Iterator that owns the data.
#[derive(Debug)]
pub(crate) struct OwningIterator<'a> {
    iterable: VecDeque<VertexId>,
    cur_idx: usize, // Quite the hack, but it works
    phantom: PhantomData<&'a u8>,
}

impl<'a> OwningIterator<'a> {
    pub fn new(iterable: VecDeque<VertexId>) -> Self {
        OwningIterator {
            iterable,
            cur_idx: 0,
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for OwningIterator<'a> {
    type Item = &'a VertexId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx == self.iterable.len() {
            None
        } else {
            let last_idx = self.cur_idx;
            self.cur_idx += 1;

            // Since we cannot annotate the lifetime 'a to &mut self
            // because of the Iterator trait's signature, this seems
            // the only way to make the compiler happy.
            //
            // TODO: If you can make this work with safe Rust, please do.
            unsafe {
                let ptr = &self.iterable[last_idx] as *const VertexId;
                let transmuted = mem::transmute::<*const VertexId, &VertexId>(ptr);
                Some(transmuted)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_yields_correct_vertex_ids() {
        let ids: VecDeque<VertexId> =
            vec![VertexId::random(), VertexId::random(), VertexId::random()]
                .iter()
                .cloned()
                .collect();
        let mut iter = OwningIterator::new(ids.clone());

        assert_eq!(iter.next(), Some(&ids[0]));
        assert_eq!(iter.next(), Some(&ids[1]));
        assert_eq!(iter.next(), Some(&ids[2]));
        assert_eq!(iter.next(), None);
    }
}
