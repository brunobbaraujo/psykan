use rayon::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use crate::asset::Asset;
use crate::graph::graph::Graph;
use crate::graph::node::Node;
use crate::traits::NodeContent;

pub struct Job<T>
where
    T: NodeContent,
{
    pub name: String,
    _graph: Graph<T>,
}

impl<T> Job<T>
where
    T: NodeContent,
{
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn graph(&self) -> &Graph<T> {
        &self._graph
    }
}
