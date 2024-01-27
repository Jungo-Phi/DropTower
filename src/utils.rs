use std::path::Path;
use eframe::egui::ColorImage;
use image::io::Reader;


pub fn load_image_from_path(path: &Path) -> ColorImage {
	let image = Reader::open(path).expect(&format!("Image {:?} not found", path)).decode().unwrap();
	let image_buffer = image.to_rgba8();
	let pixels = image_buffer.as_flat_samples();
	ColorImage::from_rgba_unmultiplied(
		[image.width() as usize, image.height() as usize],
		pixels.as_slice(),
	)
}
