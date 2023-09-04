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

use crate::benoit::fractal::Fractal;

extern crate rug;

use rug::Float;

pub mod default;
pub mod load;

pub struct Configuration {
	pub thread_count: u32,

	pub fractal: Fractal,

	pub canvas_width:  u32,
	pub canvas_height: u32,
	pub scale:         u32,
	pub frame_count:   u32,

	pub center_real:             Float,
	pub center_imaginary:        Float,
	pub zoom:                    Float,
	pub maximum_iteration_count: u32,

	pub dump_path: String,

	pub interactive: bool,
}
