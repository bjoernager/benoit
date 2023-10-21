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
use crate::benoit::image::ImageFormat;

extern crate rug;

use rug::Float;

impl Configuration {
	#[must_use]
	pub fn default() -> Configuration {
		return Configuration {
			thread_count: 0x0, // Automatic if null.

			fractal: Self::DEFAULT_FRACTAL,

			canvas_width:  0x100,
			canvas_height: 0x100,
			palette:       Self::DEFAULT_PALETTE,

			dump_path:    "./render".to_string(),
			image_format: ImageFormat::Png,

			start_frame:          0x0,
			start_centre_real:    Float::with_val(PRECISION, Self::DEFAULT_CENTRE.0),
			start_centre_imag:    Float::with_val(PRECISION, Self::DEFAULT_CENTRE.1),
			start_extra_real:     Float::with_val(PRECISION, Self::DEFAULT_EXTRA.0),
			start_extra_imag:     Float::with_val(PRECISION, Self::DEFAULT_EXTRA.1),
			start_zoom:           Float::with_val(PRECISION, Self::DEFAULT_ZOOM),
			start_max_iter_count: Self::DEFAULT_MAX_ITER_COUNT,
			start_colour_range:   Self::DEFAULT_COLOUR_RANGE,

			stop_frame:          0xF,
			stop_centre_real:    Float::with_val(PRECISION, Self::DEFAULT_CENTRE.0),
			stop_centre_imag:    Float::with_val(PRECISION, Self::DEFAULT_CENTRE.1),
			stop_extra_real:     Float::with_val(PRECISION, Self::DEFAULT_EXTRA.0),
			stop_extra_imag:     Float::with_val(PRECISION, Self::DEFAULT_EXTRA.1),
			stop_zoom:           Float::with_val(PRECISION, Self::DEFAULT_ZOOM),
			stop_max_iter_count: Self::DEFAULT_MAX_ITER_COUNT,
			stop_colour_range:   Self::DEFAULT_COLOUR_RANGE,
		};
	}
}
