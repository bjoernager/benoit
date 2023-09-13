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
use crate::benoit::fractal::Fractal;

extern crate rug;

use rug::Float;

impl Configuration {
	pub fn default() -> Configuration {
		return Configuration {
			thread_count: 0x0,

			fractal: Fractal::Mandelbrot,
			julia:   false,

			canvas_width: 0x100,
			scale:        0x2,
			frame_count:  0x10,

			centre_real:    Float::with_val(PRECISION, 0.0),
			centre_imag:    Float::with_val(PRECISION, 0.0),
			zoom:           Float::with_val(PRECISION, 1.0),
			max_iter_count: 0x100,

			dump_path: "./render/".to_string(),

			interactive: true,
		};
	}
}
