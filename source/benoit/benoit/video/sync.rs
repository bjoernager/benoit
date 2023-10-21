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

use crate::benoit::video::Video;

use std::thread::sleep;
use std::time::{Duration, Instant};

impl Video {
	pub fn sync(&self, start_frame: &Instant) -> Result<(), String> {
		let frame_duration = {
			let index = match self.canvas.window().display_index() {
				Ok( index) => index,
				Err(_)     => return Err("unable to get display index".to_string()),
			};

			let mode = match self.sdl_video.current_display_mode(index) {
				Ok( mode) => mode,
				Err(_)    => return Err("unable to get display mode".to_string()),
			};

			Duration::from_secs(0x1) / mode.refresh_rate as u32
		};

		let remaining = match frame_duration.checked_sub(start_frame.elapsed()) {
			Some(value) => value,
			None        => Duration::from_secs(0x0),
		};

		sleep(remaining);

		return Ok(());
	}
}