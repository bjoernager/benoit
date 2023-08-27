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

use crate::benoit::PRECISION;
use crate::benoit::configuration::Configuration;

extern crate rug;

use rug::Float;

impl Configuration {
	pub fn default() -> Configuration {
		return Configuration {
			thread_count: 0x0,

			canvas_width:  0x100,
			canvas_height: 0x100,
			scale:         0x1,
			frame_count:   0x10,

			center_real:             Float::with_val(PRECISION, 0.0),
			center_imaginary:        Float::with_val(PRECISION, 0.0),
			zoom:                    Float::with_val(PRECISION, 1.0),
			maximum_iteration_count: 0x100,

			dump_path: "./render/".to_string(),

			interactive: true,
		};
	}
}
