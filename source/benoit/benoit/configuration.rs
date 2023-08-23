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

pub mod load;

pub struct Configuration {
	pub thread_count: u32,

	pub canvas_width:  u32,
	pub canvas_height: u32,
	pub scale:         u32,

	pub position_x:              f64,
	pub position_y:              f64,
	pub zoom:                    f64,
	pub maximum_iteration_count: u32,

	pub dump_path: String,
}
