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

use crate::benoit::application::{Application, PreviousPosition};

extern crate sdl2;

impl Application {
	pub fn r#loop(&mut self) -> i32 {
		eprintln!();
		eprintln!("Controls:");
		eprintln!("- W      Translate up");
		eprintln!("- A      Translate left");
		eprintln!("- S      Translate down");
		eprintln!("- D      Translate right");
		eprintln!();
		eprintln!("- Q      Zoom out");
		eprintln!("- E      Zoom in");
		eprintln!();
		eprintln!("- R      Decrease max. iteration count");
		eprintln!("- F      Increase max. iteration count");
		eprintln!();
		eprintln!("- Tab    Toggle Julia");
		eprintln!("- Alt    Cycle between fractals");
		eprintln!();
		eprintln!("- Z      Print centre value (c)");
		eprintln!("- X      Dump frame");
		eprintln!("- C      Render frame");
		eprintln!();

		let mut event_pump = self.video.as_mut().unwrap().sdl.event_pump().expect("unable to get event pump");

		let canvas_size = self.canvas_height as usize * self.canvas_width as usize;

		let mut iter_count_buffer: Vec::<u32> = vec![0x0; canvas_size];
		let mut square_dist_buffer: Vec::<f32> = vec![0.0; canvas_size];

		let mut image: Vec::<u8>  = vec![0x0; canvas_size * 0x3];

		let mut previous_position = PreviousPosition {
			centre_real: self.centre_real.clone(),
			centre_imag: self.centre_imag.clone(),
			zoom:        self.zoom.clone(),
		};

		loop {
			if self.poll_events(&mut event_pump) { break }

			if self.do_render {
				self.render(&mut iter_count_buffer[..], &mut square_dist_buffer[..], &self.centre_real, &self.centre_imag, &self.zoom, self.max_iter_count);
				self.colour(&mut image[..], &iter_count_buffer[..], &square_dist_buffer[..]);

				previous_position = PreviousPosition {
					centre_real: self.centre_real.clone(),
					centre_imag: self.centre_imag.clone(),
					zoom:        self.zoom.clone(),
				};

				self.do_render = false;
			}

			self.draw(&image[..], &previous_position);

			if self.do_dump {
				let path = format!("{}/image.webp", self.dump_path);

				eprintln!("dumping image at \"{path}\"");
				self.dump(path, &image, self.canvas_width, self.canvas_height);

				self.do_dump = false;
			}
		}

		return 0x0;
	}
}