use dev::components::background_image;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = background_image()?;

    let out_dir = "./examples";
    let out_filename = "output_background_image.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
