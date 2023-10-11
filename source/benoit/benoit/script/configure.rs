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
use crate::benoit::script::Script;

impl Script {
	#[must_use]
	pub fn configure(configuration: Configuration) -> Script {
		return Script {
			fractal:  configuration.fractal,
			renderer: configuration.renderer,

			canvas_width:  configuration.canvas_width,
			canvas_height: configuration.canvas_height,
			frame_start:   configuration.frame_start,
			frame_stop:    configuration.frame_stop,

			centre: Complex::new(configuration.centre_real, configuration.centre_imag),
			zoom:   configuration.zoom,

			extra: Complex::new(configuration.extra_real, configuration.extra_imag),

			max_iter_count: configuration.max_iter_count,

			palette:      configuration.palette,
			colour_range: configuration.colour_range,

			dump_path:    configuration.dump_path,
			image_format: configuration.image_format,
		};
	}
}
