use dev::components::container;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = container()?;

    let out_dir = "./examples";
    let out_filename = "output_container.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
