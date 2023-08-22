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

use crate::benoit::application::Application;

extern crate webp;

use std::fs::write;
use webp::Encoder;

impl Application {
	pub fn dump_image(&mut self) {
		let canvas_size = self.canvas_height * self.canvas_width;

		let mut data: Vec::<u32> = vec![0x0; canvas_size as usize];
		self.render(&mut data[..]);

		let mut image: Vec::<u8> = vec![0x0; canvas_size as usize * 0x3];
		self.colour(&mut image[..], &data[..]);

		let encoder = Encoder::from_rgb(&image[..], self.canvas_width, self.canvas_height);

		let data = encoder.encode_lossless();

		let file_path = "./image.webp";

		write(file_path, &*data);
	}
}
