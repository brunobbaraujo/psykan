use crate::asset::Asset;
use crate::traits::Executable;

pub struct Job<F, R>
where
    F: FnOnce() -> R,
{
    pub name: String,
    pub assets: Vec<Asset<F, R>>,
}

impl<F, R> Job<F, R>
where
    F: Fn() -> R,
{
    pub fn new(name: String) -> Self {
        Job {
            name,
            assets: Vec::new(),
        }
    }

    pub fn add_asset(&mut self, asset: Asset<F, R>) {
        self.assets.push(asset);
    }

    pub fn execute_all(&self) -> Vec<R> {
        self.assets.iter().map(|asset| asset.execute()).collect()
    }
}
