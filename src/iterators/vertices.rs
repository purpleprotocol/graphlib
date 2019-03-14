// Copyright 2019 Octavian Oncescu

use crate::vertex_id::VertexId;

#[derive(Debug)]
pub struct VertexIter<'a> {
    current: usize,
    iterable: Vec<&'a VertexId>
}

impl<'a> VertexIter<'a> {
    pub fn new(neighbors: Vec<&'a VertexId>) -> VertexIter<'a> {
        VertexIter {
            current: 0,
            iterable: neighbors
        }
    }
}

impl<'a> Iterator for VertexIter<'a> {
    type Item = &'a VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.iterable.len() {
            return None;
        } 
        
        let result = self.iterable[self.current];
        self.current += 1;

        Some(result)
    }
}