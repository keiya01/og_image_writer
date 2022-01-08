use dev::components::white_space;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = white_space()?;

    let out_dir = "./examples";
    let out_filename = "output_white_space.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
