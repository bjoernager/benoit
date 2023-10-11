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
use crate::benoit::image::Image;
use crate::benoit::render::Render;
use crate::benoit::video::Video;

extern crate rug;

use rug::{Assign, Float};
use std::time::Instant;

impl App {
	#[must_use]
	pub fn run(mut self) -> i32 {
		let mut video = Video::initialise(self.canvas_width, self.canvas_height, self.scale);

		App::print_controls();

		let mut event_pump = video.sdl.event_pump().expect("unable to get event pump");

		let mut image  = Image::allocate( self.canvas_width, self.canvas_height);
		let mut render = Render::allocate(self.canvas_width, self.canvas_height);

		// Used for translation feedback:
		let mut prev_centre = self.centre.clone();
		let mut prev_zoom   = self.zoom.clone();

		loop {
			let frame_start = Instant::now();

			if self.poll_events(&mut event_pump) { break }

			if self.do_render {
				self.render(&mut render);

				prev_centre.assign(&self.centre);
				prev_zoom.assign(  &self.zoom);

				self.do_render = false;
			};

			image.colour(&render, self.palette, self.max_iter_count, self.colour_range);

			video.draw(&image, self.scale);
			self.draw_feedback(&mut video, &prev_centre, &prev_zoom);

			video.update();

			video.sync(&frame_start);
		}

		return 0x0;
	}

	fn render(&self, render: &mut Render) {
		eprint!("rendering...");

		let time_start = Instant::now();

		render.render(
			self.fractal,
			self.renderer,
			&self.centre,
			&self.zoom,
			&self.extra,
			self.max_iter_count,
		);

		let render_time = time_start.elapsed();

		eprintln!(" {:.3}ms", render_time.as_micros() as f32 / 1000.0);
	}

	fn draw_feedback(&self, video: &mut Video, prev_centre: &Complex, prev_zoom: &Float) {
		if {
			// Don't draw translation feedback if rendering a
			// Julia set or if we haven't done any viewport
			// translations.

			   &self.centre.real != &prev_centre.real
			|| &self.centre.imag != &prev_centre.imag
			|| &self.zoom        != prev_zoom
		}{
			video.draw_translation_feedback(self.canvas_width, self.canvas_height, self.scale, prev_centre, prev_zoom, &self.centre, &self.zoom);
		}

		if self.do_textual_feedback { video.draw_textual_feedback(&self.centre, &self.zoom, self.max_iter_count) };
	}

	fn print_controls() {
		println!("Controls:");
		println!("- \u{1B}[1mW\u{1B}[0m            Translate +Im");
		println!("- \u{1B}[1mA\u{1B}[0m            Translate -Re");
		println!("- \u{1B}[1mS\u{1B}[0m            Translate -Im");
		println!("- \u{1B}[1mD\u{1B}[0m            Translate +Re");
		println!();
		println!("- \u{1B}[1mQ\u{1B}[0m            Zoom out");
		println!("- \u{1B}[1mE\u{1B}[0m            Zoom in");
		println!();
		println!("- \u{1B}[1mR\u{1B}[0m            Decrease max. iteration count");
		println!("- \u{1B}[1mF\u{1B}[0m            Increase max. iteration count");
		println!();
		println!("- \u{1B}[1mLEFT ALT\u{1B}[0m     Cycle to previous fractal");
		println!("- \u{1B}[1mRIGHT ALT\u{1B}[0m    Cycle to next fractal");
		println!("- \u{1B}[1mTAB\u{1B}[0m          Toggle Julia");
		println!("- \u{1B}[1mLEFT CTRL\u{1B}[0m    Toggle inverse");
		println!();
		println!("- \u{1B}[1mLEFT\u{1B}[0m         Cycle to previous palette");
		println!("- \u{1B}[1mRIGHT\u{1B}[0m        Cycle to next palette");
		println!("- \u{1B}[1mUP\u{1B}[0m           Increase colour range");
		println!("- \u{1B}[1mDOWN\u{1B}[0m         Decrease colour range");
		println!();
		println!("- \u{1B}[1mF1\u{1B}[0m           Toggle textual feedback");
		println!("- \u{1B}[1mZ\u{1B}[0m            Print centre value (c)");
		println!();
		println!("- \u{1B}[1mC\u{1B}[0m            Render frame");
		println!();
		println!("Controls (holding \u{1B}[1mSHIFT\u{1B}[0m):");
		println!("- \u{1B}[1mW\u{1B}[0m            Perturbate/translate +Im");
		println!("- \u{1B}[1mA\u{1B}[0m            Perturbate/translate -Re");
		println!("- \u{1B}[1mS\u{1B}[0m            Perturbate/translate -Im");
		println!("- \u{1B}[1mD\u{1B}[0m            Perturbate/translate +Re");
		println!();
		println!("- \u{1B}[1mC\u{1B}[0m            Render frame");
		println!();
	}
}