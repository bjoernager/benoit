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
	pub fn new(buffer: &mut [u32], canvas_width: u32, canvas_height: u32, real: Float, imaginary: Float, zoom: Float, maximum_iteration_count: u32) -> RenderData {
		let buffer_pointer = buffer.as_mut_ptr();

		return RenderData {
			canvas_width:  canvas_width,
			canvas_height: canvas_height,

			real:      real,
			imaginary: imaginary,
			zoom:      zoom,

			maximum_iteration_count: maximum_iteration_count,

			buffer: buffer_pointer,
		};
	}
}
