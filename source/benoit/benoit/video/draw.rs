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

use crate::benoit::video::Video;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl Video {
	pub fn draw(&mut self, image: &[u8], canvas_width: u32, canvas_height: u32, scale: u32) {
		self.draw_image(image, canvas_width, canvas_height, scale);
	}

	fn draw_image(&mut self, image: &[u8], canvas_width: u32, canvas_height: u32, scale: u32) {
		let canvas_size = canvas_height * canvas_width;

		for pixel in 0x0..canvas_size {
			let y = pixel as u32 / canvas_width;
			let x = pixel as u32 - y * canvas_width;

			let colour = {
				let red   = image[pixel as usize * 0x3];
				let green = image[pixel as usize * 0x3 + 0x1];
				let blue  = image[pixel as usize * 0x3 + 0x2];

				Color::RGB(red, green, blue)
			};

			let square = Rect::new(
				(x * scale) as i32,
				(y * scale) as i32,
				scale,
				scale,
			);

			self.canvas.set_draw_color(colour);
			self.canvas.fill_rect(square).unwrap();
		}
	}
}
