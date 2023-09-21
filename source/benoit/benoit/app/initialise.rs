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
use crate::benoit::configuration::Configuration;
use crate::benoit::video::Video;

extern crate rayon;

use rayon::ThreadPoolBuilder;
use std::env::args;
use std::thread::available_parallelism;

impl App {
	pub fn initialise() -> App {
		let mut arguments = args();

		let configuration = match arguments.nth(0x1) {
			Some(path) => Configuration::load(path.as_str()),
			None       => Configuration::default(),
		};

		let thread_count: u32 = if configuration.thread_count == 0x0 {
			match available_parallelism() {
				Ok(ammount) => ammount.get() as u32,
				_           => 0x2,
			}
		} else {
			configuration.thread_count
		};

		eprintln!("using {thread_count} threads");

		let video = match configuration.interactive {
			true  => Some(Video::initialise(configuration.canvas_width, configuration.scale)),
			false => None,
		};

		ThreadPoolBuilder::new().num_threads(configuration.thread_count as usize).build_global().unwrap();

		eprintln!("rendering the {}", configuration.fractal.get_name());

		return App {
			thread_count: thread_count,

			fractal: configuration.fractal,
			julia:   configuration.julia,

			canvas_width: configuration.canvas_width,
			scale:        configuration.scale,
			frame_count:  configuration.frame_count,

			centre_real:    configuration.centre_real,
			centre_imag:    configuration.centre_imag,
			zoom:           configuration.zoom,
			max_iter_count: configuration.max_iter_count,

			colour_range: configuration.colour_range,

			dump_path:    configuration.dump_path,
			image_format: configuration.image_format,

			video: video,

			interactive: configuration.interactive,
			do_render:   true,
			do_dump:     false,

			row_renderer:      App::get_row_renderer(     configuration.julia),
			iterator_function: App::get_iterator_function(configuration.fractal),
		};
	}
}
