mod entities;
mod execution;
mod graph;
mod traits;
use entities::asset;
use traits::Executable;

pub fn main() {
    // Example usage of the Asset struct and Executable trait
    let asset = asset::Asset::new(
        "Example Asset".to_string(),
        vec!["example"],
        "This is an example asset".to_string(),
        || "Asset executed successfully!".to_string(),
        vec![],
    );

    // Execute the asset and print the result
    let result = asset.execute();
    println!("{}", result);
}
