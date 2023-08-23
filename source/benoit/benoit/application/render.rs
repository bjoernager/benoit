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

use std::slice;
use std::thread::{JoinHandle, spawn};
use std::time::Instant;
use std::ptr::addr_of_mut;

impl Application {
	pub fn render(&mut self, buffer: &mut [u32], position_x: f64, position_y: f64, zoom: f64, maximum_iteration_count: u32) {
		eprintln!("rendering: {}{:+}i ({}x) @ ({})", position_x, position_y, zoom, maximum_iteration_count);

		let mut threads = Vec::<JoinHandle<()>>::with_capacity(self.thread_count as usize);

		let time_start = Instant::now();

		'render_loop: {
			let get_slice = |buffer: &mut [u32], y: u32, canvas_width: u32| -> &mut [u32] {
				let slice_start = y as usize * canvas_width as usize;
				let slice = unsafe { slice::from_raw_parts_mut(addr_of_mut!(buffer[slice_start]), canvas_width as usize) };

				return slice;
			};

			let canvas_width            = self.canvas_width;
			let canvas_height           = self.canvas_height;
			let position_x              = position_x;
			let position_y              = position_y;
			let zoom                    = zoom;
			let maximum_iteration_count = maximum_iteration_count;

			let mut y: u32 = 0x0;

			for _thread in 0x0..self.thread_count {
				if y == self.canvas_height { break 'render_loop; }

				let buffer_slice = get_slice(buffer, y, self.canvas_width);

				threads.push(spawn(move || { Application::render_row(buffer_slice, y, canvas_width, canvas_height, position_x, position_y, zoom, maximum_iteration_count) }));

				y += 0x1;
			}

			for y in 0x0..self.canvas_height {
				threads.remove(0x0).join().unwrap();

				let buffer_slice = get_slice(buffer, y, self.canvas_width);

				threads.push(spawn(move || { Application::render_row(buffer_slice, y, canvas_width, canvas_height, position_x, position_y, zoom, maximum_iteration_count) }));
			}
		}

		let duration = time_start.elapsed();

		eprintln!("done ({}ms)", duration.as_millis());
	}
}
