use std::hash::{Hash, Hasher};

use crate::traits::Executable;

#[derive(Debug, Clone, PartialEq, Eq)]
struct AssetKey<'a>(Vec<&'a str>);

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
    pub name: String,
    pub key: AssetKey<'a>,
    pub description: String,
    pub dependencies: Vec<AssetKey<'a>>,
    pub func: F,
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
}

impl<'a, F, R> Executable<R> for Asset<'a, F, R>
where
    F: Fn() -> R,
{
    fn execute(&self) -> R {
        (self.func)()
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
