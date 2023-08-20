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

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl Application {
	pub fn render(&mut self) {
		println!("rendering");

		//let mut data: [i8; 0x100] = [0; 0x100];

		for y in 0x0..self.canvas_width {
			for x in 0x0..self.canvas_height {
				let pixel = y * 0xFF + x;

				let canvas_width  = self.canvas_width as f64;
				let canvas_height = self.canvas_height as f64;

				let ca = (x as f64 - canvas_width  / 2.0) / (canvas_width  / 4.0);
				let cb = (y as f64 - canvas_height / 2.0) / (canvas_height / 4.0);

				let mut za = 0.0f64;
				let mut zb = 0.0f64;

				let mut iteration_count = 0x0u32;
				while iteration_count < self.max_iteration_count {
					let distance = (za.powf(2.0) + zb.powf(2.0)).sqrt();
					if distance > 2.0 { break }

					// z = z^2 + c
					{
						// Complex square:
						// a = a^2 - b^2
						// b = 2abi
						let za_temporary = za;
						za = za.powf(2.0) - zb.powf(2.0) + ca;
						zb = za_temporary * zb * 2.0 + cb;
					}

					iteration_count += 0x1;
				}

				let value  = (iteration_count / self.max_iteration_count * 0xFF) as u8;
				let colour = Color::RGB(value, value, value);
				self.canvas.set_draw_color(colour);

				let square = Rect::new(
					x as i32,
					y as i32,
					0x1,
					0x1,
				);
				self.canvas.fill_rects(&[square]).unwrap();
			}
		}

		self.canvas.present();

		println!("done");
	}
}
