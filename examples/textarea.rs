use dev::components::textarea;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = textarea()?;

    let out_dir = "./examples";
    let out_filename = "output_textarea.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
