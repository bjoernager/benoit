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

extern crate rug;
extern crate sdl2;

use rug::Float;
use rug::ops::PowAssign;
use std::time::Instant;

impl App {
	pub fn animate(&self) -> i32 {
		let canvas_size = self.canvas_width as usize * self.canvas_width as usize;

		let mut iter_count_buffer:  Vec::<u32> = vec![0x0; canvas_size];
		let mut square_dist_buffer: Vec::<f32> = vec![0.0; canvas_size];

		let mut image: Vec::<u8>  = vec![0x0; canvas_size * 0x3];

		if self.frame_count == 0x1 {
			// If only a single frame is to be rendered, we
			// aren't going to animate it.

			eprint!("rendering at {}{:+}i ({}x)...", self.centre_real.to_f32(), self.centre_imag.to_f32(), self.zoom.to_f32());

			let time_start = Instant::now();

			self.render(&mut iter_count_buffer[..], &mut square_dist_buffer[..], &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count);
			let render_time = time_start.elapsed();

			self.colour(&mut image[..], self.max_iter_count, &mut iter_count_buffer[..], &mut square_dist_buffer[..]);
			let colour_time = time_start.elapsed() - render_time;

			self.dump(format!("{}/render.webp", self.dump_path), &image, self.canvas_width);

			eprintln!(" rend. {:.3}ms, col. {:.3}ms", render_time.as_micros() as f32 / 1000.0, colour_time.as_micros() as f32 / 1000.0);

			return 0x0;
		}

		// zoom_start:
		let mut zoom = Float::with_val(PRECISION, 1.0 / 4.0);

		let zoom_stop  = Float::with_val(PRECISION, &self.zoom);

		let zoom_factor = {
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
			//
			// N.b. that we subtract one from frame_count as we
			// want the final render to have exactly the
			// specified zoom.

			let exponent = Float::with_val(PRECISION, 1.0 / (self.frame_count as f32 - 1.0));

			let mut factor = Float::with_val(PRECISION, &zoom_stop / &zoom);
			factor.pow_assign(exponent);

			factor
		};

		eprintln!("animating {} frames at {}{:+}i to {:.3} (fac. {:.3})", self.frame_count, self.centre_real.to_f64(), self.centre_imag.to_f64(), zoom_stop.to_f64(), zoom_factor.to_f64());

		for frame in 0x0..self.frame_count {
			eprint!("{frame:010} ({:.3}x)...", zoom.to_f32());

			let time_start = Instant::now();

			self.render(&mut iter_count_buffer[..], &mut square_dist_buffer[..], &self.centre_real, &self.centre_imag, &zoom, self.max_iter_count);
			let render_time = time_start.elapsed();

			self.colour(&mut image[..], self.max_iter_count, &mut iter_count_buffer[..], &mut square_dist_buffer[..]);
			let colour_time = time_start.elapsed() - render_time;

			self.dump(format!("{}/frame{frame:010}.webp", self.dump_path), &image, self.canvas_width);

			eprintln!(" rend. {:.3}ms, col. {:.3}ms", render_time.as_micros() as f32 / 1000.0, colour_time.as_micros() as f32 / 1000.0);

			zoom *= &zoom_factor;
		}

		return 0x0;
	}
}