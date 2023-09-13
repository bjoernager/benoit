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

use crate::benoit::render_data::RenderData;

extern crate rug;

use rug::Float;

impl RenderData {
	pub fn new(iter_count_buffer: &mut [u32], square_dist_buffer: &mut [f32], canvas_width: u32, centre_real: Float, centre_imag: Float, zoom: Float, max_iter_count: u32) -> RenderData {
		let iter_count_buffer_pointer  = iter_count_buffer.as_mut_ptr();
		let square_dist_buffer_pointer = square_dist_buffer.as_mut_ptr();

		return RenderData {
			canvas_width: canvas_width,

			centre_real: centre_real,
			centre_imag: centre_imag,
			zoom:        zoom,

			max_iter_count: max_iter_count,

			iter_count_buffer: iter_count_buffer_pointer,
			square_dist_buffer: square_dist_buffer_pointer,
		};
	}
}
