use dev::components::encode;
use std::fs::write;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let buf = encode()?;

    let out_dir = "./examples";
    let out_filename = "output_encode.jpg";

    write(Path::new(&format!("{}/{}", out_dir, out_filename)), &buf)?;

    Ok(())
}
