use dev::components::absolute;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = absolute()?;

    let out_dir = "./examples";
    let out_filename = "output_absolute.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
