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

extern crate rug;

use rug::Float;

pub mod new;
pub mod slice;

pub struct RenderData {
	pub canvas_width: u32,

	pub centre_real: Float,
	pub centre_imag: Float,
	pub zoom:        Float,

	pub max_iter_count: u32,

	iter_count_buffer:  *mut u32,
	square_dist_buffer: *mut f32,
}

unsafe impl Send for RenderData {}
unsafe impl Sync for RenderData {}
