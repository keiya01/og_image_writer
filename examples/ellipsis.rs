use dev::components::ellipsis;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = ellipsis()?;

    let out_dir = "./examples";
    let out_filename = "output_ellipsis.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
