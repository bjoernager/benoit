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
use crate::benoit::rendering::Rendering;
use crate::benoit::video::Video;

extern crate rug;

use rug::{Assign, Float};
use std::time::Instant;

impl App {
	pub fn interactive(&mut self) -> i32 {
		assert_eq!(self.interactive, true);
		let mut video = self.video.take().unwrap();

		App::print_controls();

		let mut event_pump = video.sdl.event_pump().expect("unable to get event pump");

		let (mut iter_count_buffer, mut square_dist_buffer, mut image) = App::allocate_buffers(self.canvas_width, self.canvas_height);

		// Used for colouring:
		let mut prev_max_iter_count     = self.max_iter_count;
		let mut prev_multibrot_exponent = self.multibrot_exponent;

		// Used for translation feedback:
		let mut prev_centre_real    = self.centre_real.clone();
		let mut prev_centre_imag    = self.centre_imag.clone();
		let mut prev_zoom           = self.zoom.clone();

		loop {
			let frame_start = Instant::now();

			if self.poll_events(&mut event_pump) { break }

			if self.do_render {
				eprint!("rendering...");

				let time_start = Instant::now();

				self.render(&mut iter_count_buffer[..], &mut square_dist_buffer[..], &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count);
				let render_time = time_start.elapsed();

				eprintln!(" {:.3}ms", render_time.as_micros() as f32 / 1000.0);

				prev_max_iter_count     = self.max_iter_count;
				prev_multibrot_exponent = self.multibrot_exponent;

				prev_centre_real.assign(&self.centre_real);
				prev_centre_imag.assign(&self.centre_imag);
				prev_zoom.assign(&self.zoom);

				self.do_render = false;
			}

			self.colour(&mut image[..], prev_multibrot_exponent, prev_max_iter_count.min(self.max_iter_count), &mut iter_count_buffer[..], &mut square_dist_buffer[..]);

			video.draw(&image[..], self.canvas_width, self.canvas_height, self.scale);
			self.draw_feedback(&mut video, &prev_centre_real, &prev_centre_imag, &prev_zoom);

			video.update();

			video.sync(&frame_start);
		}

		return 0x0;
	}

	pub fn draw_feedback(&self, video: &mut Video, prev_centre_real: &Float, prev_centre_imag: &Float, prev_zoom: &Float) {
		let julia = match self.rendering {
			Rendering::Julia => true,
			_                => false,
		};

		if {
			// Don't draw translation feedback if rendering a
			// Julia set or if we haven't done any viewport
			// translations.

			   !julia
			&& (&self.centre_real != prev_centre_real
			||  &self.centre_imag != prev_centre_imag
			||  &self.zoom        != prev_zoom)
		}{
			let feedback_info = FeedbackInfo {
				prev_centre_real: prev_centre_real,
				prev_centre_imag: prev_centre_imag,
				prev_zoom:        prev_zoom,
				next_centre_real: &self.centre_real,
				next_centre_imag: &self.centre_imag,
				next_zoom:        &self.zoom,
			};

			video.draw_translation_feedback(self.canvas_width, self.canvas_height, self.scale, &feedback_info);
		}

		if self.do_textual_feedback { video.draw_textual_feedback(&self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count) };
	}

	pub fn print_controls() {
		println!("Controls:");
		println!("- \u{1B}[1mW\u{1B}[0m            Translate up");
		println!("- \u{1B}[1mA\u{1B}[0m            Translate left");
		println!("- \u{1B}[1mS\u{1B}[0m            Translate down");
		println!("- \u{1B}[1mD\u{1B}[0m            Translate right");
		println!();
		println!("- \u{1B}[1mQ\u{1B}[0m            Zoom out");
		println!("- \u{1B}[1mE\u{1B}[0m            Zoom in");
		println!();
		println!("- \u{1B}[1mR\u{1B}[0m            Decrease max. iteration count");
		println!("- \u{1B}[1mF\u{1B}[0m            Increase max. iteration count");
		println!();
		println!("- \u{1B}[1mTab\u{1B}[0m          Toggle Julia");
		println!("- \u{1B}[1mLeft Alt\u{1B}[0m     Cycle to previous fractal");
		println!("- \u{1B}[1mRight Alt\u{1B}[0m    Cycle to next fractal");
		println!();
		println!("- \u{1B}[1mUp\u{1B}[0m           Increase colour range");
		println!("- \u{1B}[1mDown\u{1B}[0m         Decrease colour range");
		println!();
		println!("- \u{1B}[1mLeft\u{1B}[0m         Cycle to previous palette");
		println!("- \u{1B}[1mRight\u{1B}[0m        Cycle to next palette");
		println!();
		println!("- \u{1B}[1mF1\u{1B}[0m           Toggle textual feedback");
		println!("- \u{1B}[1mC\u{1B}[0m            Print centre value (c)");
		println!();
		println!("- \u{1B}[1mSpace\u{1B}[0m        Render frame");
		println!();
	}
}