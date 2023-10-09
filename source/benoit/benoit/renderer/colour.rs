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

use crate::benoit::palette::{Palette, PALETTE_DATA_LENGTH};
use crate::benoit::render::colour_data::ColourData;
use crate::benoit::renderer::Renderer;

extern crate rayon;

use rayon::prelude::*;

impl Renderer {
	pub fn colour(
		self,
		buffer:             &mut [u8],
		palette:            Palette,
		canvas_width:       u32,
		canvas_height:      u32,
		multibrot_exponent: f32,
		max_iter_count:     u32,
		colour_range:       f32,
		iter_count_buffer:  &[u32],
		square_dist_buffer: &[f32],
	) {
		let data = ColourData::new(buffer, canvas_width, canvas_height, multibrot_exponent, max_iter_count, colour_range, palette, iter_count_buffer, square_dist_buffer);

		let (canvas_size, overflow) = canvas_height.overflowing_mul(canvas_width);
		if overflow { panic!("overflow when calculating canvas size") };

		(0x0..canvas_size).into_par_iter().for_each(|index| {
			colour_point(&data, index as usize);
		});
	}
}

fn colour_point(data: &ColourData, index: usize) {
	let (iter_count_buffer, square_dist_buffer) = data.input_buffers();

	let image = data.output_buffers();

	let (exponent, max_iter_count, colour_range, palette_data) = data.consts();

	let iter_count = unsafe { *iter_count_buffer.get_unchecked( index) };
	let distance   = unsafe { *square_dist_buffer.get_unchecked(index) }.sqrt();

	let (red, green, blue) = if iter_count < max_iter_count {
		let factor = (iter_count as f32 + 1.0 - distance.log(exponent).log(exponent)) / colour_range % 1.0;

		let index = (factor * PALETTE_DATA_LENGTH as f32).round() as usize;
		unsafe { *palette_data.get_unchecked(index) }
	} else {
		(0.0, 0.0, 0.0)
	};

	let red   = (red   * 255.0).round() as u8;
	let green = (green * 255.0).round() as u8;
	let blue  = (blue  * 255.0).round() as u8;

	unsafe {
		let index = index * 0x3;

		*image.get_unchecked_mut(index)       = red;
		*image.get_unchecked_mut(index + 0x1) = green;
		*image.get_unchecked_mut(index + 0x2) = blue;
	}
}
