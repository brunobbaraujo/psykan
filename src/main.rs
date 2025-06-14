mod asset;
mod traits;

use asset::Asset;
use traits::Executable;

fn main() {
    // Create an asset with a more complex closure
    let complex_asset = Asset::new(
        "Complex Asset".to_string(),
        vec!["complex".to_string()],
        "Performs calculation".to_string(),
        || {
            let x = 10;
            let y = 5;
            x * y + 2
        },
    );
    println!("Complex asset result: {}", complex_asset.execute());
}
