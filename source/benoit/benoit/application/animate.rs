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

use crate::benoit::application::Application;

extern crate rug;
extern crate sdl2;

use rug::Float;
use rug::ops::PowAssign;
use std::ops::MulAssign;

impl Application {
	pub fn animate(&self) -> i32 {
		let stop_zoom = Float::with_val(self.precision, &self.zoom);

		let zoom_factor = {
			// To get the zoom factor, we first want the 'a'
			// value of the growth function from (0) and to
			// (frame_count) on the x-dimension and from (1) to
			// (zoom) on the y-dimension:
			//
			// a = nroot(x1-x0, y1/y0)
			//
			// but this may be simplified for use with Rug
			// because
			//
			// nroot(a, b) = b^(1/a)
			//
			// making the final equation
			//
			// (x1-x0)^(1/y1*y0) = (zoom)^(1/frame_count)

			let x_difference = Float::with_val(self.precision, self.frame_count);

			let exponent = Float::with_val(self.precision, 1.0 / &x_difference);

			let mut factor = Float::with_val(self.precision, &stop_zoom);
			factor.pow_assign(exponent);

			factor
		};

		let mut zoom = Float::with_val(self.precision, 1.0);

		eprintln!("animating {} frames at {}{:+} to {:.3} (fac.: {:.3})", self.frame_count, self.center_real.to_f64(), self.center_imaginary.to_f64(), stop_zoom.to_f64(), zoom_factor.to_f64());

		let canvas_size = self.canvas_height as usize * self.canvas_width as usize;

		let mut data:  Vec::<u32> = vec![0x0; canvas_size];
		let mut image: Vec::<u8>  = vec![0x0; canvas_size * 0x3];

		for frame in 0x0..self.frame_count {
			eprint!("{frame:010}: ");
			self.render(&mut data[..], self.precision, &self.center_real, &self.center_imaginary, &zoom, self.maximum_iteration_count);
			self.colour(&mut image[..], &data[..]);

			self.dump(format!("{}/frame{frame:010}.webp", self.dump_path), &image, self.canvas_width, self.canvas_height);

			zoom.mul_assign(&zoom_factor);
		}

		return 0x0;
	}
}
