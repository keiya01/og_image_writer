use dev::components::*;
use std::path::Path;

macro_rules! snapshot {
    ($component_name:ident) => {{
        let mut w = $component_name().unwrap();

        let out_dir = "./snapshots";
        let out_filename = format!("output_{}.png", stringify!($component_name));

        w.generate(Path::new(&format!("{}/{}", out_dir, out_filename)))
            .unwrap();
    }};
}

#[test]
#[ignore]
fn update_snapshots() {
    snapshot!(absolute);
    snapshot!(background_color);
    snapshot!(background_image);
    snapshot!(container);
    snapshot!(ellipsis);
    snapshot!(font_context);
    snapshot!(font_kern);
    snapshot!(row_container);
    snapshot!(textarea);
    snapshot!(white_space);
}
