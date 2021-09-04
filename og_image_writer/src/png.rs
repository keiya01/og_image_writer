use std::fs::File;

pub fn get_png_data(src: &str) -> (Vec<u8>, png::OutputInfo) {
  // open the image file
  let file = File::open(src)
  .expect("Failed to open the file");

  // parse the metadata
  let decoder = png::Decoder::new(file);
  let mut reader = decoder.read_info().expect("Invalid PNG");

  // extract the raw image
  let mut buf = vec![0; reader.output_buffer_size()];

  let info = reader.next_frame(&mut buf)
    .expect("Invalid PNG");

  (buf, info)
}