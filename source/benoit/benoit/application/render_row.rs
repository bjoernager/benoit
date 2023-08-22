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

impl Application {
	pub fn render_row(data: &mut [u32], y: u32, canvas_width: u32, canvas_height: u32, position_x: f64, position_y: f64, zoom: f64, maximum_iteration_count: u32) {
		for x in 0x0..canvas_width {
			let canvas_width  = canvas_width as f64;
			let canvas_height = canvas_height as f64;

			let ca = (x as f64 - canvas_width  / 2.0) / canvas_width  * 4.0 / zoom + position_x;
			let cb = (y as f64 - canvas_height / 2.0) / canvas_height * 4.0 / zoom + position_y;

			let mut za: f64 = 0.0;
			let mut zb: f64 = 0.0;

			let mut iteration_count: u32 = 0x0;
			while iteration_count < maximum_iteration_count {
				let square_distance = za * za + zb * zb;
				if square_distance > 2.0 * 2.0 { break }

				{
					// z = z^2 + c

					// Complex square:
					// a = a^2 - b^2
					// b = 2abi

					let za_temporary = za;
					za = za * za - zb * zb + ca;
					zb = za_temporary * zb * 2.0 + cb;
				}

				iteration_count += 0x1;
			}

			data[x as usize] = iteration_count;
		}
	}
}
