use dev::components::into_vec;
use image::{save_buffer, ColorType};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let (w, h, buf) = into_vec()?;

    let out_dir = "./examples";
    let out_filename = "output_into_vec.png";

    save_buffer(
        Path::new(&format!("{}/{}", out_dir, out_filename)),
        &buf,
        w,
        h,
        ColorType::Rgba8,
    )
    .expect("Could not save image");

    Ok(())
}
