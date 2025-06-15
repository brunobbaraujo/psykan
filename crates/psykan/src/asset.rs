use crate::traits::Executable;

pub struct Asset<F, R>
where
    F: Fn() -> R,
{
    pub name: String,
    pub key: Vec<String>,
    pub description: String,
    pub func: F,
}

impl<F, R> Asset<F, R>
where
    F: Fn() -> R,
{
    pub fn new(name: String, key: Vec<String>, description: String, func: F) -> Self {
        Asset {
            name,
            key,
            description,
            func,
        }
    }
}

impl<F, R> Executable<R> for Asset<F, R>
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
            vec!["test".to_string()],
            "This is a test asset".to_string(),
            || "Test closure executed".to_string(),
        );

        assert_eq!(asset.name, "Test Asset");
        assert_eq!(asset.key, vec!["test"]);
        assert_eq!(asset.description, "This is a test asset");
    }

    #[test]
    fn test_asset_execution() {
        let asset = Asset::new(
            "Execution Test".to_string(),
            vec!["execute".to_string()],
            "This asset tests execution".to_string(),
            || "Execution successful!".to_string(),
        );

        assert_eq!(asset.execute(), "Execution successful!");
    }

    #[test]
    fn test_asset_with_different_return_types() {
        // Asset returning an integer
        let int_asset = Asset::new(
            "Integer Asset".to_string(),
            vec!["integer".to_string()],
            "This asset returns an integer".to_string(),
            || 42,
        );
        assert_eq!(int_asset.execute(), 42);

        // Asset returning a boolean
        let bool_asset = Asset::new(
            "Boolean Asset".to_string(),
            vec!["boolean".to_string()],
            "This asset returns a boolean".to_string(),
            || true,
        );
        assert_eq!(bool_asset.execute(), true);
    }
}
