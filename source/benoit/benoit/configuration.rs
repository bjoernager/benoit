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

use crate::benoit::fractal::{Fractal, FractalKind};
use crate::benoit::image::ImageFormat;
use crate::benoit::palette::Palette;

extern crate rug;

use rug::Float;

pub mod default;
pub mod load;

pub struct Configuration {
	pub thread_count: u32,

	pub fractal: Fractal,

	pub canvas_width:  u32,
	pub canvas_height: u32,
	pub scale:         u32,
	pub frame_start:   u32,
	pub frame_stop:    u32,

	pub centre_real: Float,
	pub centre_imag: Float,
	pub zoom:        Float,

	pub extra_real: Float,
	pub extra_imag: Float,

	pub max_iter_count: u32,

	pub palette:      Palette,
	pub colour_range: f32,

	pub dump_path:    String,
	pub image_format: ImageFormat,
}

impl Configuration {
	pub const DEFAULT_FRACTAL: Fractal = Fractal::new(FractalKind::Mandelbrot, false, false);

	pub const DEFAULT_CENTRE: (f64, f64) = (0.0, 0.0);
	pub const DEFAULT_EXTRA:  (f64, f64) = (0.0, 0.0);
	pub const DEFAULT_ZOOM:   f64        = 1.0;

	pub const DEFAULT_MAX_ITER_COUNT: u32 = 0x100;

	pub const DEFAULT_PALETTE:      Palette = Palette::Fire;
	pub const DEFAULT_COLOUR_RANGE: f32     = 64.0;
}
