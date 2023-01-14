// Copyright 2019 Octavian Oncescu

use crate::vertex_id::VertexId;
#[cfg(not(feature = "std"))]
use core::fmt::Debug;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(feature = "std")]
use std::fmt::Debug;

pub(crate) trait MergedTrait<'a>: Iterator<Item = &'a VertexId> + Debug {}

impl<'a, T> MergedTrait<'a> for T where T: Iterator<Item = &'a VertexId> + Debug {}

/// Generic Vertex Iterator.
#[derive(Debug)]
pub struct VertexIter<'a>(pub(crate) Box<dyn 'a + MergedTrait<'a>>);

impl<'a> Iterator for VertexIter<'a> {
    type Item = &'a VertexId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
