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

use rug::Float;
use std::ops::{AddAssign, DivAssign, MulAssign};

impl Application {
	pub fn render_row(data: &mut [u32], precision: u32, y: u32, canvas_width: u32, canvas_height: u32, center_real: Float, center_imaginary: Float, zoom: Float, maximum_iteration_count: u32) {
		let two = Float::with_val(precision, 2.0);

		for x in 0x0..canvas_width {
			let canvas_width  = Float::with_val(precision, canvas_width);
			let canvas_height = Float::with_val(precision, canvas_height);

			//let ca = Float::with_val(precision, (&x - &canvas_width  / 2.0) / &canvas_width  * 4.0 / &zoom + &center_real);
			//let cb = Float::with_val(precision, (&y - &canvas_height / 2.0) / &canvas_height * 4.0 / &zoom + &center_imaginary);

			let x_float = Float::with_val(precision, x);
			let y_float = Float::with_val(precision, y);

			// Re(c) = (x-canvas_width/2)/canvas_width*4/zoom+Re(z)

			let ca = {
				let tmp0 = Float::with_val(precision, &canvas_width / 2.0);

				let mut ca = Float::with_val(precision, &x_float - &tmp0);
				ca.div_assign(&canvas_width);
				ca.mul_assign(4.0);
				ca.div_assign(&zoom);
				ca.add_assign(&center_real);

				ca
			};

			// Im(c) = (x-canvas_height/2)/canvas_height*4/zoom+Im(z)

			let cb = {
				let tmp0 = Float::with_val(precision, &canvas_height / 2.0);

				let mut cb = Float::with_val(precision, &y_float - &tmp0);
				cb.div_assign(&canvas_height);
				cb.mul_assign(4.0);
				cb.div_assign(&zoom);
				cb.add_assign(&center_imaginary);

				cb
			};

			let mut za = Float::with_val(precision, &ca);
			let mut zb = Float::with_val(precision, &cb);

			let mut iteration_count: u32 = 0x0;
			while iteration_count < maximum_iteration_count {
				let square_distance = Float::with_val(precision, &za * &za + &zb * &zb);
				if square_distance > 4.0 { break }

				{
					// z = z^2 + c

					// Complex square:
					// a = a^2 - b^2
					// b = 2abi

					let za_temporary = Float::with_val(precision, &za);

					za = Float::with_val(precision, &za * &za - &zb * &zb);
					za = Float::with_val(precision, &za + &ca);

					zb = Float::with_val(precision, &za_temporary * &zb);
					zb = Float::with_val(precision, &zb * &two + &cb);
				}

				iteration_count += 0x1;
			}

			unsafe { *data.get_unchecked_mut(x as usize) = iteration_count }
		}
	}
}
