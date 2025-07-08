use crate::entities::asset::ID_SEPARATOR;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::asset::Asset;
use crate::graph::graph::Graph;
use crate::graph::node::Node;
use crate::traits::NodeContent;

pub struct UnresolvedJob<F, R>
where
    F: Send + Sync + Fn() -> R,
{
    pub name: String,
    pub assets: Vec<Asset<F, R>>,
}

impl<F, R> UnresolvedJob<F, R>
where
    F: Send + Sync + Fn() -> R,
{
    pub fn new(name: String, assets: Vec<Asset<F, R>>) -> Self {
        UnresolvedJob { name, assets }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn assets(&self) -> &Vec<Asset<F, R>> {
        &self.assets
    }

    fn check_dependencies(&self) -> Result<(), String> {
        let all_dependencies = self
            .assets
            .iter()
            .flat_map(|asset| asset.dependencies())
            .collect::<HashSet<_>>();

        let asset_keys = self
            .assets
            .iter()
            .map(|asset| asset.key())
            .collect::<HashSet<_>>();

        let missing_deps = all_dependencies
            .difference(&asset_keys)
            .cloned()
            .collect::<Vec<_>>();

        if !missing_deps.is_empty() {
            return Err(format!(
                "Missing dependencies for job '{}': {:?}",
                self.name, missing_deps
            ));
        }

        Ok(())
    }

    pub fn resolve(self) -> Job<Asset<F, R>> {
        let mut graph = Graph::new();
        self.check_dependencies().expect("Dependency check failed");

        Job {
            name: self.name,
            _graph: graph,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{asset::Asset, entities::asset::AssetKey};

    #[test]
    fn test_unresolved_job_creation() {
        let asset1 = Asset::new(
            "Asset1".to_string(),
            vec!["asset1".to_string()],
            "First asset".to_string(),
            || "Result 1".to_string(),
            vec![],
        );

        let asset2 = Asset::new(
            "Asset1".to_string(),
            vec!["asset1".to_string()],
            "First asset".to_string(),
            || "Result 1".to_string(),
            vec![AssetKey::new(vec!["asset2".to_string()])],
        );

        let job = UnresolvedJob::new("TestJob".to_string(), vec![asset1, asset2]);
        assert_eq!(job.name(), "TestJob");
        assert_eq!(job.assets().len(), 2);
    }
}
