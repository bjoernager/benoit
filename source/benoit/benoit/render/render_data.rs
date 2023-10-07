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

use crate::benoit::{PRECISION, width_height_ratio};

extern crate rug;

use rug::Float;
use std::slice::from_raw_parts_mut;

pub struct RenderData {
	canvas_width:  u32,
	canvas_height: u32,

	canvas_size: usize,

	centre_real: Float,
	centre_imag: Float,
	zoom:        Float,

	max_iter_count: u32,

	inverse: bool,

	x_offset: f32,
	y_offset: f32,

	x_factor: f32,
	y_factor: f32,

	iter_count_buffer:  *mut u32,
	square_dist_buffer: *mut f32,
}

impl RenderData {
	#[must_use]
	pub fn new(iter_count_buffer: &mut [u32], square_dist_buffer: &mut [f32], canvas_width: u32, canvas_height: u32, centre_real: Float, centre_imag: Float, zoom: Float, max_iter_count: u32, inverse: bool) -> RenderData {
		let (width_ratio, height_ratio) = width_height_ratio(canvas_width, canvas_height);

		return RenderData {
			canvas_width:  canvas_width,
			canvas_height: canvas_height,

			canvas_size: canvas_height as usize * canvas_width as usize,

			centre_real: centre_real,
			centre_imag: centre_imag,
			zoom:        zoom,

			max_iter_count: max_iter_count,

			inverse: inverse,

			x_offset: canvas_width  as f32 / -2.0,
			y_offset: canvas_height as f32 / -2.0,

			x_factor: 4.0 / canvas_width  as f32 * width_ratio,
			y_factor: 4.0 / canvas_height as f32 * height_ratio,

			iter_count_buffer:  iter_count_buffer.as_mut_ptr(),
			square_dist_buffer: square_dist_buffer.as_mut_ptr(),
		};
	}

	#[must_use]
	pub fn inverse_factor(&self, a: &Float, b: &Float) -> Float {
		return if self.inverse {
			let mut inverse_factor = Float::with_val(PRECISION, a * a);
			inverse_factor += b * b;
			inverse_factor.recip_mut();

			inverse_factor
		} else {
			Float::with_val(PRECISION, 0x1)
		};
	}

	#[must_use]
	pub fn canvas_size(&self) -> (u32, u32) {
		return (self.canvas_width, self.canvas_height);
	}

	#[must_use]
	pub fn input(&self) -> (&Float, &Float, &Float, u32) {
		return (&self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count);
	}

	#[must_use]
	pub fn output_buffers(&self) -> (&mut [u32], &mut [f32]) {
		let iter_count  = unsafe { from_raw_parts_mut(self.iter_count_buffer,  self.canvas_size as usize) };
		let square_dist = unsafe { from_raw_parts_mut(self.square_dist_buffer, self.canvas_size as usize) };

		return (iter_count, square_dist);
	}

	#[must_use]
	pub fn consts(&self) -> (f32, f32, f32, f32) {
		return (self.x_offset, self.y_offset, self.x_factor, self.y_factor);
	}
}

unsafe impl Send for RenderData {}
unsafe impl Sync for RenderData {}
