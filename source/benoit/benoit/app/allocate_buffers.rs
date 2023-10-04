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

impl App {
	pub fn allocate_buffers(canvas_width: u32, canvas_height: u32) -> (Vec::<u32>, Vec::<f32>, Vec::<u8>) {
		let canvas_size = canvas_height as usize * canvas_width as usize;

		let iter_count_buffer:  Vec::<u32> = vec![0x0; canvas_size];
		let square_dist_buffer: Vec::<f32> = vec![0.0; canvas_size];

		let image: Vec::<u8>  = vec![0x0; canvas_size * 0x3];

		return (iter_count_buffer, square_dist_buffer, image);
	}
}
