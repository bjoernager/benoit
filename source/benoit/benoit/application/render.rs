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
use std::slice;
use std::thread::{JoinHandle, spawn};
use std::time::Instant;
use std::ptr::addr_of_mut;

impl Application {
	pub fn render(&mut self) {
		eprintln!("rendering: {}{:+}i ({}x) @ ({})", self.position_x, self.position_y, self.zoom, self.maximum_iteration_count);

		let canvas_size = self.canvas_height * self.canvas_width;

		let mut data = vec![0x0; canvas_size as usize];

		let mut threads = Vec::<JoinHandle<()>>::with_capacity(self.thread_count as usize);

		let time_start = Instant::now();

		'render_loop: {
			let mut y: u32 = 0x0;

			for _thread in 0x0..self.thread_count {
				if y == self.canvas_height { break 'render_loop; }

				let slice_start = y as usize * self.canvas_width as usize;
				let data_slice  = unsafe { slice::from_raw_parts_mut(addr_of_mut!(data[slice_start]), self.canvas_width as usize) };

				let canvas_width            = self.canvas_width;
				let canvas_height           = self.canvas_height;
				let position_x              = self.position_x;
				let position_y              = self.position_y;
				let zoom                    = self.zoom;
				let maximum_iteration_count = self.maximum_iteration_count;

				threads.push(spawn(move || { Application::render_row(data_slice, y, canvas_width, canvas_height, position_x, position_y, zoom, maximum_iteration_count) }));

				y += 0x1;
			}

			for y in 0x0..self.canvas_height {
				threads.remove(0x0).join();

				let slice_start = y as usize * self.canvas_width as usize;
				let data_slice  = unsafe { slice::from_raw_parts_mut(addr_of_mut!(data[slice_start]), self.canvas_width as usize) };

				let canvas_width            = self.canvas_width;
				let canvas_height           = self.canvas_height;
				let position_x              = self.position_x;
				let position_y              = self.position_y;
				let zoom                    = self.zoom;
				let maximum_iteration_count = self.maximum_iteration_count;

				threads.push(spawn(move || { Application::render_row(data_slice, y, canvas_width, canvas_height, position_x, position_y, zoom, maximum_iteration_count) }));
			}
		}

		let duration = time_start.elapsed();

		for thread in threads {
			thread.join().unwrap();
		}

		for pixel in 0x0..canvas_size {
			let y = pixel as u32 / self.canvas_width;
			let x = pixel as u32 - y * self.canvas_width;

			let iteration_count = data[pixel as usize];

			let factor = {
				let factor = iteration_count as f32 / 64.0 % 1.0;

				(if factor >= 1.0 / 2.0 {
					1.0 - factor
				} else {
					factor
				}) * 2.0
			};

			let value: u8 = if iteration_count != self.maximum_iteration_count {
				(factor * 255.0).round() as u8
			} else {
				0x0
			};
			let colour = Color::RGB(value, value, value);
			self.canvas.set_draw_color(colour);

			let rectangle = Rect::new(
				(x * self.scale) as i32,
				(y * self.scale) as i32,
				self.scale,
				self.scale
			);
			self.canvas.fill_rects(&[rectangle]).unwrap();
		}

		self.canvas.present();

		eprintln!("done ({}ms)", duration.as_millis());
	}
}
