use dev::components::font_context;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let mut w = font_context()?;

    let out_dir = "./examples";
    let out_filename = "output_font_context.png";

    w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))?;

    Ok(())
}
