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
		eprintln!("- Tab    Cycle between fractals");
		eprintln!();
		eprintln!("- Z      Print centre value (c)");
		eprintln!("- X      Dump frame");
		eprintln!("- C      Render frame");
		eprintln!();

		let mut event_pump = self.video.as_mut().unwrap().sdl.event_pump().expect("unable to get event pump");

		let canvas_size = self.canvas_height as usize * self.canvas_width as usize;

		let mut iteration_count_buffer: Vec::<u32> = vec![0x0; canvas_size];
		let mut square_distance_buffer: Vec::<f32> = vec![0.0; canvas_size];

		let mut image: Vec::<u8>  = vec![0x0; canvas_size * 0x3];

		loop {
			if self.poll_events(&mut event_pump) { break }

			if self.do_draw {
				self.render(&mut iteration_count_buffer[..], &mut square_distance_buffer[..], &self.center_real, &self.center_imaginary, &self.zoom, self.maximum_iteration_count);
				self.colour(&mut image[..], &iteration_count_buffer[..], &square_distance_buffer[..]);

				self.draw(&image[..]);

				self.do_draw = false;
			}

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