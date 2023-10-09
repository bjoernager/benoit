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
use crate::benoit::complex::Complex;
use crate::benoit::configuration::Configuration;
use crate::benoit::video::Video;

extern crate rayon;

use rayon::ThreadPoolBuilder;
use std::env::args;
use std::thread::available_parallelism;

impl App {
	#[must_use]
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
			true  => Some(Video::initialise(configuration.canvas_width, configuration.canvas_height, configuration.scale)),
			false => None,
		};

		ThreadPoolBuilder::new().num_threads(configuration.thread_count as usize).build_global().unwrap();

		return App {
			thread_count: thread_count,

			fractal:  configuration.fractal,
			renderer: configuration.renderer,

			canvas_width:  configuration.canvas_width,
			canvas_height: configuration.canvas_height,
			scale:         configuration.scale,
			frame_start:   configuration.frame_start,
			frame_stop:    configuration.frame_stop,

			centre: Complex { real: configuration.centre_real, imag: configuration.centre_imag },
			zoom:   configuration.zoom,

			extra: Complex { real: configuration.extra_real, imag: configuration.extra_imag },

			max_iter_count: configuration.max_iter_count,

			palette:      configuration.palette,
			colour_range: configuration.colour_range,

			dump_path:    configuration.dump_path,
			image_format: configuration.image_format,

			interactive:         configuration.interactive,
			do_render:           true,
			do_textual_feedback: false,

			video: video,
		};
	}
}
