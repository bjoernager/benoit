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

use crate::benoit::render::Render;

impl Render {
	pub fn allocate(canvas_width: u32, canvas_height: u32) -> Render {
		let canvas_size = {
			let (canvas_size, overflow) = canvas_height.overflowing_mul(canvas_width);
			if overflow { panic!("overflow when calculating canvas size") };

			canvas_size as usize
		};

		let iter_count_buffer:  Vec::<u32> = vec![0x0; canvas_size];
		let square_dist_buffer: Vec::<f32> = vec![0.0; canvas_size];

		return Render {
			canvas_width:  canvas_width,
			canvas_height: canvas_height,

			info: None,

			iter_count_buffer:  iter_count_buffer,
			square_dist_buffer: square_dist_buffer,
		};
	}
}
