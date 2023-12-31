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
use crate::benoit::fractal::Fractal;
use crate::benoit::image::ImageFormat;
use crate::benoit::palette::Palette;

extern crate rug;

use rug::Float;

pub mod animate;
pub mod configure;
pub mod dump_frame;
pub mod run;
pub mod still;

pub struct Keyframe {
	frame:          u32,
	centre:         Complex,
	extra:          Complex,
	zoom:           Float,
	max_iter_count: u32,
	colour_range:   f32,
}

pub struct Script {
	// Configuration:
	fractal:  Fractal,

	canvas_width:  u32,
	canvas_height: u32,

	palette: Palette,

	dump_path:    String,
	image_format: ImageFormat,

	start: Keyframe,
	stop:  Keyframe,
}
