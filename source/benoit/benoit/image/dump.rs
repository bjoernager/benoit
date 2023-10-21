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

use crate::benoit::image::{Image, ImageFormat};

extern crate png;
extern crate webp;

use std::fs::{File, write};
use std::io::BufWriter;

impl Image {
	pub fn dump(&self, path: &str, format: ImageFormat) -> Result<(), String> {
		use ImageFormat::*;

		return match format {
			Png  => self.dump_png( path),
			Webp => self.dump_webp(path),
		};
	}

	fn dump_png(&self, path: &str) -> Result<(), String> {
		let path = path.to_owned() + ".png";

		let file = match File::create(path) {
			Ok( file) => file,
			Err(_)    => return Err("unable to create file".to_string()),
		};
		let file_buffer = BufWriter::new(file);

		let mut encoder = png::Encoder::new(file_buffer, self.width, self.height);
		encoder.set_color(png::ColorType::Rgb);
		encoder.set_depth(png::BitDepth::Eight);
		encoder.set_compression(png::Compression::Fast);
		encoder.set_srgb(png::SrgbRenderingIntent::Perceptual);

		let mut writer = match encoder.write_header() {
			Ok( writer) => writer,
			Err(_)      => return Err("unable to write image header".to_string()),
		};
		if writer.write_image_data(self.raw()).is_err() { return Err("unable to write image".to_string()) };

		return Ok(());
	}

	fn dump_webp(&self, path: &str) -> Result<(), String> {
		let path = path.to_owned() + ".webp";

		let encoder = webp::Encoder::from_rgb(self.raw(), self.width, self.height);
		let data    = encoder.encode_lossless();

		if write(path, &*data).is_err() { return Err("unable to write image".to_string()) };

		return Ok(());
	}
}
