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

use crate::benoit::app::App;
use crate::benoit::render::{colour, render};

use std::time::Instant;

impl App {
	pub fn still(&self) -> i32 {
		assert_eq!(self.frame_count, 0x1);

		let (mut iter_count_buffer, mut square_dist_buffer, mut image) = App::allocate_buffers(self.canvas_width, self.canvas_height);

		eprint!("rendering at {}{:+}i ({}x)...", self.centre_real.to_f32(), self.centre_imag.to_f32(), self.zoom.to_f32());
		let time_start = Instant::now();

		render(&mut iter_count_buffer[..], &mut square_dist_buffer[..], self.canvas_width, self.canvas_height, &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count, self.inverse, self.point_renderer, self.iterator_function);
		let render_time = time_start.elapsed();

		eprint!(" {:.3}ms, colouring...", render_time.as_micros() as f32 / 1000.0);

		colour(&mut image[..], self.canvas_width, self.canvas_height, self.multibrot_exponent, self.max_iter_count, self.colour_range, self.palette, &iter_count_buffer[..], &square_dist_buffer[..]);
		let colour_time = time_start.elapsed() - render_time;

		eprint!(" {:.3}ms...", colour_time.as_micros() as f32 / 1000.0);

		let path = App::image_filename(format!("{}/render.webp", self.dump_path).as_str(), self.image_format);
		self.dump(path.as_str(), &image, self.canvas_width, self.canvas_height);

		eprintln!(" done");

		return 0x0;
	}
}
