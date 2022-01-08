use dev::components::row_container;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = row_container()?;

    let out_dir = "./examples";
    let out_filename = "output_row_container.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
