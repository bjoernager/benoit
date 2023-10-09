/*
	Copyright 2021, 2023 Gabriel Bjørnager Jensen.

	This file is part of Benoit.

	Benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::benoit::ImageFormat;
use crate::benoit::app::App;

extern crate png;
extern crate webp;

use std::fs::{File, write};
use std::io::BufWriter;

impl App {
	pub fn dump(path: &str, image: &[u8], canvas_width: u32, canvas_height: u32, image_format: ImageFormat) {
		match image_format {
			ImageFormat::Png  => dump_png( path, image, canvas_width, canvas_height),
			ImageFormat::Webp => dump_webp(path, image, canvas_width, canvas_height),
		}
	}
}

fn dump_png(path: &str, image: &[u8], canvas_width: u32, canvas_height: u32) {
	let path = path.to_owned() + ".png";

	let file        = File::create(path).expect("unable to create file");
	let file_buffer = BufWriter::new(file);

	let mut encoder = png::Encoder::new(file_buffer, canvas_width, canvas_height);
	encoder.set_color(png::ColorType::Rgb);
	encoder.set_depth(png::BitDepth::Eight);
	encoder.set_compression(png::Compression::Fast);
	encoder.set_srgb(png::SrgbRenderingIntent::Perceptual);

	let mut writer = encoder.write_header().expect("unable to write image");
	writer.write_image_data(image).expect("unable to write image");
}

fn dump_webp(path: &str, image: &[u8], canvas_width: u32, canvas_height: u32) {
	let path = path.to_owned() + ".webp";

	let encoder = webp::Encoder::from_rgb(&image[..], canvas_width, canvas_height);

	let data = encoder.encode_lossless();

	write(path, &*data).expect("unable to write image");
}
