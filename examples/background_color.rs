use dev::components::background_color;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = background_color()?;

    let out_dir = "./examples";
    let out_filename = "output_background_color.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
