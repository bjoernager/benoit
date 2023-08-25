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

use crate::benoit::configuration::Configuration;

impl Configuration {
	pub fn default() -> Configuration {
		return Configuration {
			thread_count: 0x2,

			canvas_width:  0x100,
			canvas_height: 0x100,
			scale:         0x1,

			position_x:              0.0,
			position_y:              0.0,
			zoom:                    1.0,
			maximum_iteration_count: 0x100,

			dump_path: "./image.webp".to_string(),
		};
	}
}
