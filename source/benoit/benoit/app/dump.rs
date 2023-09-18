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

use crate::benoit::app::App;

extern crate webp;

use std::fs::write;
use webp::Encoder;

impl App {
	pub fn dump(&self, path: String, image: &[u8], canvas_width: u32) {
		let encoder = Encoder::from_rgb(&image[..], canvas_width, canvas_width);

		let data = encoder.encode_lossless();

		write(path, &*data).expect("unable to write image");
	}
}
