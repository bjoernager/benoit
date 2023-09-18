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

use crate::benoit::FeedbackInfo;
use crate::benoit::app::App;

extern crate rug;

use rug::Assign;
use std::time::Instant;

impl App {
	pub fn r#loop(&mut self) -> i32 {
		assert_eq!(self.video.is_some(), true);

		eprintln!();
		eprintln!("Controls:");
		eprintln!("- W    Translate up");
		eprintln!("- A    Translate left");
		eprintln!("- S    Translate down");
		eprintln!("- D    Translate right");
		eprintln!();
		eprintln!("- Q    Zoom out");
		eprintln!("- E    Zoom in");
		eprintln!();
		eprintln!("- R    Decrease max. iteration count");
		eprintln!("- F    Increase max. iteration count");
		eprintln!();
		eprintln!("- Tab  Toggle Julia");
		eprintln!("- Alt  Cycle between fractals");
		eprintln!();
		eprintln!("- Up   Increase colour range");
		eprintln!("- Down Decrease colour range");
		eprintln!();
		eprintln!("- Z    Print centre value (c)");
		eprintln!("- X    Dump frame");
		eprintln!("- C    Render frame");
		eprintln!();

		let mut event_pump = unsafe { self.video.as_mut().unwrap_unchecked().sdl.event_pump().expect("unable to get event pump") };

		let canvas_size = self.canvas_width as usize * self.canvas_width as usize;

		let mut iter_count_buffer:  Vec::<u32> = vec![0x0; canvas_size];
		let mut square_dist_buffer: Vec::<f32> = vec![0.0; canvas_size];

		let mut image: Vec::<u8>  = vec![0x0; canvas_size * 0x3];

		let mut prev_centre_real    = self.centre_real.clone();
		let mut prev_centre_imag    = self.centre_imag.clone();
		let mut prev_zoom           = self.zoom.clone();
		let mut prev_max_iter_count = self.max_iter_count;

		loop {
			let frame_start = Instant::now();

			if self.poll_events(&mut event_pump) { break }

			if self.do_render {
				eprint!("rendering...");

				let time_start = Instant::now();

				self.render(&mut iter_count_buffer[..], &mut square_dist_buffer[..], &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count);
				let render_time = time_start.elapsed();

				eprintln!(" rend. {:.3}ms", render_time.as_micros() as f32 / 1000.0);

				prev_centre_real.assign(&self.centre_real);
				prev_centre_imag.assign(&self.centre_imag);
				prev_zoom.assign(&self.zoom);
				prev_max_iter_count = self.max_iter_count;

				self.do_render = false;
			}

			self.colour(&mut image[..], prev_max_iter_count, &mut iter_count_buffer[..], &mut square_dist_buffer[..]);

			{
				let feedback_info = FeedbackInfo {
					prev_centre_real: &prev_centre_real,
					prev_centre_imag: &prev_centre_imag,
					prev_zoom:        &prev_zoom,
					next_centre_real: &self.centre_real,
					next_centre_imag: &self.centre_imag,
					next_zoom:        &self.zoom,
				};

				unsafe { self.video.as_mut().unwrap_unchecked().draw(&image[..], self.canvas_width, self.scale, Some(&feedback_info)) };
			}

			if self.do_dump {
				let path = format!("{}/image.webp", self.dump_path);

				eprintln!("dumping image at \"{path}\"");
				self.dump(path, &image, self.canvas_width);

				self.do_dump = false;
			}

			unsafe { self.video.as_ref().unwrap_unchecked().sync(&frame_start) };
		}

		return 0x0;
	}
}