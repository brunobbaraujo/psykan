use std::hash::{Hash, Hasher};

use crate::traits::NodeContent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetKey<'a>(Vec<&'a str>);

impl<'a> AssetKey<'a> {
    pub fn new(key: Vec<&'a str>) -> Self {
        AssetKey(key)
    }

    pub fn to_vec(&self) -> Vec<&'a str> {
        self.0.clone()
    }
}

impl Hash for AssetKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub struct Asset<'a, F, R>
where
    F: FnOnce() -> R,
{
    name: String,
    key: AssetKey<'a>,
    description: String,
    dependencies: Vec<AssetKey<'a>>,
    func: F,
}

impl<'a, F, R> Asset<'a, F, R>
where
    F: Fn() -> R,
{
    pub fn new(
        name: String,
        key: Vec<&'a str>,
        description: String,
        func: F,
        dependencies: Vec<AssetKey<'a>>,
    ) -> Self {
        Asset {
            name: name,
            key: AssetKey::new(key),
            description: description,
            func: func,
            dependencies: dependencies,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn key(&self) -> &AssetKey<'a> {
        &self.key
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn dependencies(&self) -> &Vec<AssetKey<'a>> {
        &self.dependencies
    }

    pub fn func(&self) -> &F {
        &self.func
    }
}

impl<'a, F, R> NodeContent for Asset<'a, F, R>
where
    F: Fn() -> R,
{
    type Output = R;

    fn id(&self) -> String {
        self.key().to_vec().join("__")
    }

    fn dependencies(&self) -> Vec<String> {
        self.dependencies
            .iter()
            .map(|d| d.to_vec().join("."))
            .collect()
    }

    fn execute(&self) -> Self::Output {
        (self.func())()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_creation() {
        let asset = Asset::new(
            "Test Asset".to_string(),
            vec!["test"],
            "This is a test asset".to_string(),
            || "Test closure executed".to_string(),
            vec![],
        );

        assert_eq!(asset.name, "Test Asset");
        assert_eq!(asset.key, AssetKey(vec!["test"]));
        assert_eq!(asset.description, "This is a test asset");
    }

    #[test]
    fn test_asset_execution() {
        let asset = Asset::new(
            "Execution Test".to_string(),
            vec!["execute"],
            "This asset tests execution".to_string(),
            || "Execution successful!".to_string(),
            vec![],
        );

        assert_eq!(asset.execute(), "Execution successful!");
    }

    #[test]
    fn test_asset_with_different_return_types() {
        // Asset returning an integer
        let int_asset = Asset::new(
            "Integer Asset".to_string(),
            vec!["integer"],
            "This asset returns an integer".to_string(),
            || 42,
            vec![],
        );
        assert_eq!(int_asset.execute(), 42);

        // Asset returning a boolean
        let bool_asset = Asset::new(
            "Boolean Asset".to_string(),
            vec!["boolean"],
            "This asset returns a boolean".to_string(),
            || true,
            vec![],
        );
        assert_eq!(bool_asset.execute(), true);
    }
}
