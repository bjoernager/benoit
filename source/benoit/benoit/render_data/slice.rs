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

use std::slice::from_raw_parts_mut;

impl RenderData {
	pub unsafe fn slice(&self, row: u32) -> (&mut [u32], &mut [f32]) {
		let offset = row as isize * self.canvas_width as isize;
		let iteration_count = from_raw_parts_mut(self.iteration_count_buffer.offset(offset), self.canvas_width as usize);
		let distance        = from_raw_parts_mut(self.square_distance_buffer.offset(offset), self.canvas_width as usize);

		return (iteration_count, distance);
	}
}
