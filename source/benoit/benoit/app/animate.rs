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

use crate::benoit::{ImageFormat, PRECISION};
use crate::benoit::app::App;
use crate::benoit::complex::Complex;
use crate::benoit::fractal::Fractal;
use crate::benoit::palette::Palette;
use crate::benoit::renderer::Renderer;

extern crate rug;
extern crate sdl2;

use rug::Float;
use rug::ops::PowAssign;
use std::time::Instant;

impl App {
	pub fn animate(&self) -> i32 {
		let frame_count = self.frame_stop - self.frame_start;

		if frame_count == 0x0 { return self.still() };

		let (mut iter_count_buffer, mut square_dist_buffer, mut image) = App::allocate_buffers(self.canvas_width, self.canvas_height);

		// zoom_start:
		let mut zoom = Float::with_val(PRECISION, 1.0 / 4.0);

		let zoom_stop  = Float::with_val(PRECISION, &self.zoom);

		let zoom_factor = get_zoom_factor(&zoom, &zoom_stop, self.frame_stop);

		zoom = if self.frame_start > 0x0 {
			let mut zoom = zoom_factor.clone();
			zoom.pow_assign(frame_count);

			zoom
		} else {
			zoom
		};

		eprintln!("animating from #{} to #{} ({} frame(s)) at {}{:+}i to {:.3}x (fac. ~{:.3})", self.frame_start, self.frame_stop, frame_count + 0x1, self.centre.real.to_f32(), self.centre.imag.to_f32(), zoom_stop.to_f32(), zoom_factor.to_f32());

		for frame in 0x0..=frame_count {
			let frame_name = format!("frame{frame:010}");

			dump_frame(
				self.dump_path.as_str(),
				frame_name.as_str(),
				&mut image[..],
				&mut iter_count_buffer[..],
				&mut square_dist_buffer[..],
				self.renderer,
				&self.fractal,
				self.palette,
				self.canvas_width,
				self.canvas_height,
				&self.centre,
				&self.extra,
				&zoom,
				self.max_iter_count,
				self.colour_range,
				self.image_format,
			);

			zoom *= &zoom_factor;
		}

		return 0x0;
	}

	fn still(&self) -> i32 {
		let (mut iter_count_buffer, mut square_dist_buffer, mut image) = App::allocate_buffers(self.canvas_width, self.canvas_height);

		const FRAME_NAME: &str = "render";

		dump_frame(
			self.dump_path.as_str(),
			FRAME_NAME,
			&mut image[..],
			&mut iter_count_buffer[..],
			&mut square_dist_buffer[..],
			self.renderer,
			&self.fractal,
			self.palette,
			self.canvas_width,
			self.canvas_height,
			&self.centre,
			&self.extra,
			&self.zoom,
			self.max_iter_count,
			self.colour_range,
			self.image_format,
		);

		return 0x0;
	}
}

fn dump_frame(
	dump_path:          &str,
	name:               &str,
	image:              &mut [u8],
	iter_count_buffer:  &mut [u32],
	square_dist_buffer: &mut [f32],
	renderer:           Renderer,
	fractal:            &Fractal,
	palette:            Palette,
	canvas_width:       u32,
	canvas_height:      u32,
	centre:             &Complex,
	extra:              &Complex,
	zoom:               &Float,
	max_iter_count:     u32,
	colour_range:       f32,
	image_format:       ImageFormat,
) {
	eprint!("\"{name}\" (2^{:.9}x)...", zoom.to_f64().log2());

	let time_start = Instant::now();

	renderer.render(
		iter_count_buffer,
		square_dist_buffer,
		fractal,
		canvas_width,
		canvas_height,
		centre,
		zoom,
		extra,
		max_iter_count,
	);

	let render_time = time_start.elapsed();
	eprint!(" {:.3}ms, colouring...", render_time.as_micros() as f32 / 1000.0);

	renderer.colour(
		image,
		palette,
		canvas_width,
		canvas_height,
		fractal.exponent(),
		max_iter_count,
		colour_range,
		iter_count_buffer,
		square_dist_buffer,
	);

	let colour_time = time_start.elapsed() - render_time;
	eprint!(" {:.3}ms...", colour_time.as_micros() as f32 / 1000.0);

	let path = format!("{dump_path}/{name}");

	App::dump(path.as_str(), &image, canvas_width, canvas_height, image_format);
	eprintln!(" done");
}

fn get_zoom_factor(zoom_start: &Float, zoom_stop: &Float, frame_count: u32) -> Float {
	assert!(frame_count > 0x0);

	// To get the zoom factor, we first want the 'a'
	// value of the growth function from (0) to
	// (frame_count) on the x-dimension and from
	// (zoom_start) to (zoom_stop) on the y-dimension:
	//
	// a = nroot(x1-x0,y1/y0),
	//
	// but this may be simplified for use with Rug
	// because
	//
	// nroot(a,b) = b^(1/a),
	//
	// making the final equation
	//
	// (y1/y0)^(1/(x1-x0)) = (zoom_stop/zoom_start)^(1/(frame_count-1)).

	let frame_start: f32 = 0.0;
	let frame_stop:  f32 = frame_count as f32;

	let exponent = Float::with_val(PRECISION, 1.0 / (frame_stop - frame_start));

	let mut factor = Float::with_val(PRECISION, zoom_stop / zoom_start);
	factor.pow_assign(exponent);

	factor
}
