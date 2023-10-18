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

use crate::benoit::colour_data::ColourData;
use crate::benoit::image::Image;
use crate::benoit::palette::{Palette, PALETTE_DATA_LENGTH};
use crate::benoit::render::Render;

extern crate rayon;

use rayon::prelude::*;

impl Image {
	pub fn colour(&mut self, render: &Render, palette: Palette, new_max_iter_count: u32, colour_range: f32) {
		if render.canvas_size() != self.size() { panic!("canvas size mismatch") };

		let (fractal, max_iter_count) = render.info().expect("cannot colour before render");

		let data = ColourData::new(
			self,
			fractal.exponent(),
			max_iter_count.min(new_max_iter_count),
			colour_range,
			palette.data(),
		);

		self.data.par_iter_mut().for_each(|point| {
			let index = data.index(point);
			let (iter_count, dist) = render[index];
			*point = colour_point(&data, iter_count, dist);
		});
	}
}

fn colour_point(data: &ColourData, iter_count: u32, dist: f32) -> (u8, u8, u8) {
	let (exponent, max_iter_count, colour_range, palette_data) = data.consts();

	let (red, green, blue) = if iter_count < max_iter_count {
		let factor = (iter_count as f32 + 1.0 - dist.ln().log(exponent)) / colour_range;

		let index = (factor * PALETTE_DATA_LENGTH as f32).round() as usize % PALETTE_DATA_LENGTH;
		palette_data[index]
	} else {
		(0.0, 0.0, 0.0)
	};

	let red   = (red   * 255.0).round() as u8;
	let green = (green * 255.0).round() as u8;
	let blue  = (blue  * 255.0).round() as u8;

	return (red, green, blue);
}
