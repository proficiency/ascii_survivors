#[path = "../maps/mod.rs"]
mod maps;
#[path = "../resources/mod.rs"]
mod resources;

use anyhow::Result;
use maps::map::{create_test_map_for_level, map_to_xp};
use resources::level::Level;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<()> {
    let map = create_test_map_for_level(Level::Grassland);
    let xp = map_to_xp(&map);

    let output_path = PathBuf::from("assets/maps/grassland.xp");
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&output_path)?;
    xp.write(&mut file)?;

    println!("Wrote {}", output_path.display());
    Ok(())
}
