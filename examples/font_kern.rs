use dev::components::font_kern;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = font_kern()?;

    let out_dir = "./examples";
    let out_filename = "output_font_kern.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
