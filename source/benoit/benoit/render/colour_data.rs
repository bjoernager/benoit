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

use std::slice::{from_raw_parts, from_raw_parts_mut};

pub struct ColourData {
	pub canvas_width: u32,

	pub exponent:       f32,
	pub max_iter_count: u32,
	pub colour_range:   f32,

	iter_count_buffer:  *const u32,
	square_dist_buffer: *const f32,

	image: *mut u8,
}

impl ColourData {
	pub fn new(image: &mut [u8], canvas_width: u32, exponent: f32, max_iter_count: u32, colour_range: f32, iter_count_buffer: &[u32], square_dist_buffer: &[f32]) -> ColourData {
		return ColourData {
			canvas_width: canvas_width,

			exponent:       exponent,
			max_iter_count: max_iter_count,
			colour_range:   colour_range,

			iter_count_buffer:  iter_count_buffer.as_ptr(),
			square_dist_buffer: square_dist_buffer.as_ptr(),

			image: image.as_mut_ptr(),
		};
	}

	pub unsafe fn slice(&self, row: u32) -> (&[u32], &[f32], &mut [u8]) {
		let offset = row as isize * self.canvas_width as isize;

		let iter_count = from_raw_parts(self.iter_count_buffer.offset(offset), self.canvas_width as usize);
		let dist       = from_raw_parts(self.square_dist_buffer.offset(offset), self.canvas_width as usize);

		let offset = offset * 0x3;

		let image = from_raw_parts_mut(self.image.offset(offset), self.canvas_width as usize * 0x3);

		return (iter_count, dist, image);
	}
}

unsafe impl Send for ColourData {}
unsafe impl Sync for ColourData {}
