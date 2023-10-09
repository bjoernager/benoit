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

use crate::benoit::palette::{Palette, PaletteData};

use std::slice::{from_raw_parts, from_raw_parts_mut};

pub struct ColourData {
	canvas_size: usize,

	exponent:       f32,
	max_iter_count: u32,
	colour_range:   f32,

	palette_data: &'static PaletteData,

	iter_count_buffer:  *const u32,
	square_dist_buffer: *const f32,

	image: *mut u8,
}

impl ColourData {
	#[must_use]
	pub fn new(image: &mut [u8], canvas_width: u32, canvas_height: u32, exponent: f32, max_iter_count: u32, colour_range: f32, palette: Palette, iter_count_buffer: &[u32], square_dist_buffer: &[f32]) -> ColourData {
		return ColourData {
			canvas_size: canvas_height as usize * canvas_width as usize,

			exponent:       exponent,
			max_iter_count: max_iter_count,
			colour_range:   colour_range,

			palette_data: palette.get_data(),

			iter_count_buffer:  iter_count_buffer.as_ptr(),
			square_dist_buffer: square_dist_buffer.as_ptr(),

			image: image.as_mut_ptr(),
		};
	}

	#[must_use]
	pub fn input_buffers(&self) -> (&[u32], &[f32]) {
		let iter_count = unsafe { from_raw_parts(self.iter_count_buffer,  self.canvas_size) };
		let dist       = unsafe { from_raw_parts(self.square_dist_buffer, self.canvas_size) };

		return (iter_count, dist);
	}

	#[must_use]
	pub fn output_buffers(&self) -> &mut [u8] {
		let image = unsafe { from_raw_parts_mut(self.image, self.canvas_size * 0x3) };

		return image;
	}

	#[must_use]
	pub fn consts(&self) -> (f32, u32, f32, &'static PaletteData) {
		return (self.exponent, self.max_iter_count, self.colour_range, self.palette_data);
	}
}

unsafe impl Send for ColourData {}
unsafe impl Sync for ColourData {}
