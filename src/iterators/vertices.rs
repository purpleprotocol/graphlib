// Copyright 2019 Octavian Oncescu

use crate::vertex_id::VertexId;
use std::fmt::Debug;

pub(crate) trait MergedTrait<'a,>: Iterator<Item = &'a VertexId> + Debug {}

impl<'a, T,> MergedTrait<'a,> for T
    where T: Iterator<Item = &'a VertexId> + Debug {}

/// Generic Vertex Iterator.
#[derive(Debug)]
pub struct VertexIter<'a>(pub(crate) Box<dyn 'a + MergedTrait<'a,>>,);

impl<'a> Iterator for VertexIter<'a> {
    type Item = &'a VertexId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
