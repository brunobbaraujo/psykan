use rayon::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use crate::asset::Asset;
use crate::graph::graph::Graph;
use crate::graph::node::Node;

pub struct Job<'a, F, R>
where
    F: FnOnce() -> R,
{
    pub name: String,
    _graph: Graph<'a, R>,
}

impl<'a, F, R> Job<'a, F, R>
where
    F: FnOnce() -> R,
{
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn graph(&self) -> &Graph<'a, R> {
        &self._graph
    }
}

/// Represents a job that can be partially constructed with assets
pub struct PartialJob<'a, F, R>
where
    F: FnOnce() -> R,
{
    pub name: String,
    pub assets: Vec<Asset<'a, F, R>>,
}

impl<'a, F, R> PartialJob<'a, F, R>
where
    F: FnOnce() -> R,
{
    pub fn new(name: String, assets: Vec<Asset<'a, F, R>>) -> Self {
        PartialJob {
            name,
            assets: assets,
        }
    }

    pub fn add_asset(&mut self, asset: Asset<'a, F, R>) {
        self.assets.push(asset);
    }

    pub fn resolve(&self) -> Job<'a, F, R> {
        let mut inverse_dependencies = HashMap::new();
        self.assets.into_iter().for_each(|asset| {
            asset.dependencies.iter().for_each(|dep| {
                inverse_dependencies
                    .entry(dep.clone())
                    .or_insert_with(Vec::new)
                    .push(asset.key.clone());
            })
        });
        let root_nodes = inverse_dependencies
            .into_par_iter()
            .filter_map(|(key, deps)| if deps.len() > 1 { Some(key) } else { None })
            .collect::<Vec<_>>();
        let mut graph: Graph<&Asset<F, R>> = Graph::new();
        for asset in &self.assets {
            let node = Node::new(asset.key.to_vec(), Some(asset));
            if root_nodes.contains(&asset.key) {
                graph.add_root_node(node.clone());
            }
        }

        Job {
            name: self.name.clone(),
            _graph: graph,
        }
    }
}
