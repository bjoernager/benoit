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
use crate::benoit::image::{Image, ImageFormat};
use crate::benoit::palette::Palette;
use crate::benoit::render::Render;
use crate::benoit::script::Script;

extern crate rug;

use rug::Float;
use std::time::Instant;

impl Script {
	pub(super) fn dump(
		dump_path:      &str,
		name:           &str,
		image:          &mut Image,
		render:         &mut Render,
		fractal:        Fractal,
		palette:        Palette,
		centre:         &Complex,
		extra:          &Complex,
		zoom:           &Float,
		max_iter_count: u32,
		colour_range:   f32,
		image_format:   ImageFormat,
	) {
		eprint!("\"{name}\" (2^{:.9}x)...", zoom.to_f64().log2());

		let time_start = Instant::now();

		render.render(
			fractal,
			centre,
			zoom,
			extra,
			max_iter_count,
		);

		let render_time = time_start.elapsed();
		eprint!(" {:.3}ms, colouring...", render_time.as_micros() as f32 / 1000.0);

		image.colour(&render, palette, max_iter_count, colour_range);

		let colour_time = time_start.elapsed() - render_time;
		eprint!(" {:.3}ms...", colour_time.as_micros() as f32 / 1000.0);

		let path = format!("{dump_path}/{name}");

		image.dump(path.as_str(), image_format);
		eprintln!(" done");
	}
}
