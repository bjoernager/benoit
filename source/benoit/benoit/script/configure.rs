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

use crate::benoit::complex::Complex;
use crate::benoit::configuration::Configuration;
use crate::benoit::script::{Keyframe, Script};

impl Script {
	#[must_use]
	pub fn configure(configuration: Configuration) -> Script {
		return Script {
			fractal: configuration.fractal,

			canvas_width:  configuration.canvas_width,
			canvas_height: configuration.canvas_height,
			palette:       configuration.palette,

			dump_path:    configuration.dump_path,
			image_format: configuration.image_format,

			start: Keyframe {
				frame:          configuration.start_frame,
				centre:         Complex::new(configuration.start_centre_real, configuration.start_centre_imag),
				extra:          Complex::new(configuration.start_extra_real,  configuration.start_extra_imag),
				zoom:           configuration.start_zoom,
				max_iter_count: configuration.start_max_iter_count,
				colour_range:   configuration.start_colour_range,
			},

			stop: Keyframe {
				frame:          configuration.stop_frame,
				centre:         Complex::new(configuration.stop_centre_real, configuration.stop_centre_imag),
				extra:          Complex::new(configuration.stop_extra_real,  configuration.stop_extra_imag),
				zoom:           configuration.stop_zoom,
				max_iter_count: configuration.stop_max_iter_count,
				colour_range:   configuration.stop_colour_range,
			},
		};
	}
}
