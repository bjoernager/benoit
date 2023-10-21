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
use crate::benoit::app::App;
use crate::benoit::complex::Complex;
use crate::benoit::configuration::Configuration;

extern crate rug;

use rug::Float;

impl App {
	#[must_use]
	pub fn new(canvas_width: u32, canvas_height: u32) -> App {
		return App {
			fractal: Configuration::DEFAULT_FRACTAL,

			canvas_width:  canvas_width,
			canvas_height: canvas_height,
			scale:         0x2,

			centre: Complex::new(Float::with_val(PRECISION, Configuration::DEFAULT_CENTRE.0), Float::with_val(PRECISION, Configuration::DEFAULT_CENTRE.1)),
			extra:  Complex::new(Float::with_val(PRECISION, Configuration::DEFAULT_EXTRA.0),  Float::with_val(PRECISION, Configuration::DEFAULT_EXTRA.1)),
			zoom:   Float::with_val(PRECISION, Configuration::DEFAULT_ZOOM),

			max_iter_count: Configuration::DEFAULT_MAX_ITER_COUNT,

			palette:      Configuration::DEFAULT_PALETTE,
			colour_range: Configuration::DEFAULT_COLOUR_RANGE,

			do_render:           true,
			do_textual_feedback: false,
		};
	}
}
