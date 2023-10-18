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
			canvas_height: 0xC0,
			scale:         0x2,
			frame_start:   0x0,
			frame_stop:    0x0,

			centre_real: Float::with_val(PRECISION, Self::DEFAULT_CENTRE.0),
			centre_imag: Float::with_val(PRECISION, Self::DEFAULT_CENTRE.1),
			zoom:        Float::with_val(PRECISION, Self::DEFAULT_ZOOM),

			extra_real: Float::with_val(PRECISION, Self::DEFAULT_EXTRA.0),
			extra_imag: Float::with_val(PRECISION, Self::DEFAULT_EXTRA.1),

			max_iter_count: Self::DEFAULT_MAX_ITER_COUNT,

			palette:      Self::DEFAULT_PALETTE,
			colour_range: Self::DEFAULT_COLOUR_RANGE,

			dump_path:    "./render".to_string(),
			image_format: ImageFormat::Png,
		};
	}
}
