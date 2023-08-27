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
use std::slice;
use std::thread::{JoinHandle, spawn};
use std::time::Instant;
use std::ptr::addr_of_mut;

impl Application {
	pub fn render(&self, buffer: &mut [u32], center_real: &Float, center_imaginary: &Float, zoom: &Float, maximum_iteration_count: u32) {
		eprint!("rendering...");

		let mut threads = Vec::<JoinHandle<()>>::with_capacity(self.thread_count as usize);

		let time_start = Instant::now();

		'render_loop: {
			// We render the set using one thread per row.

			let get_slice = |buffer: &mut [u32], y: u32, canvas_width: u32| -> &mut [u32] {
				let slice_start = y as usize * canvas_width as usize;
				let slice = unsafe { slice::from_raw_parts_mut(addr_of_mut!(buffer[slice_start]), canvas_width as usize) };

				return slice;
			};

			let canvas_width            = self.canvas_width;
			let canvas_height           = self.canvas_height;

			// Firstly, we fill up the thread vector by
			// spawning threads.

			let mut y: u32 = 0x0;
			for _thread in 0x0..self.thread_count {
				// We should stop if there are no remaining rows.
				if y == self.canvas_height { break 'render_loop; }

				let buffer_slice = get_slice(buffer, y, self.canvas_width);

				let center_real = center_real.clone();
				let center_imaginary = center_imaginary.clone();
				let zoom       = zoom.clone();

				threads.push(spawn(move || { Application::render_row(buffer_slice, y, canvas_width, canvas_height, center_real, center_imaginary, zoom, maximum_iteration_count) }));

				y += 0x1;
			}

			// Secondly, we continuously join the first thread
			// to spawn another one. This is as to make the
			// threads more "active" by making the take on more
			// rows when they complete.

			for y in 0x0..self.canvas_height {
				threads.remove(0x0).join().unwrap();

				let buffer_slice = get_slice(buffer, y, self.canvas_width);

				let center_real = center_real.clone();
				let center_imaginary = center_imaginary.clone();
				let zoom       = zoom.clone();

				threads.push(spawn(move || { Application::render_row(buffer_slice, y, canvas_width, canvas_height, center_real, center_imaginary, zoom, maximum_iteration_count) }));
			}
		}

		let duration = time_start.elapsed();

		eprintln!(" done ({}ms)", duration.as_millis());
	}
}
