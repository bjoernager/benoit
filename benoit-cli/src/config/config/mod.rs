/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit-cli.

	benoit-cli is free software: you can redistrib-
	ute it and/or modify it under the terms of the
	GNU General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit-cli is distributed in the hope that it
	will be useful, but WITHOUT ANY WARRANTY; with-
	out even the implied warranty of MERCHANTABILITY
	or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU Gene-
	ral Public License along with benoit-cli. If
	not, see <https://www.gnu.org/licenses/>.
*/

mod load;
mod load_from;
mod print_help;

use crate::error::Error;
use crate::keyframe::Keyframe;

use benoit::fractal::Fractal;
use benoit::palette::Palette;
use std::path::PathBuf;

/// For configuring animations.
///
/// A configuration can be loaded from a file using [`load_from`](Config::load_from).
///
/// An example of a configuration is the following:
///
/// ```toml
/// # This renders a single frame.
///
/// [render]
/// count  = 1
/// width  = 1024
/// height = 1024
///
/// fractal = "mandelbrot" # multibrot2
/// inverse = false
/// julia   = false
///
/// [render.start]
/// frame = 0
///
/// max_iter_count = 0x100
///
/// centre = "0.0+0.0i"
/// seed   = "0.0+0.0i"
/// zoom   = "1.0"
///
/// colour_range = 64.0
///
/// [render.stop]
/// frame = 0
///
/// max_iter_count = 0x100
///
/// centre = "0.0+0.0i"
/// seed   = "0.0+0.0i"
/// zoom   = "1.0"
///
/// colour_range = 64.0
///
/// [final]
/// palette = "fire"
///
/// [output]
/// directory = "render/"
/// ```
#[derive(Clone, Debug)]
pub struct Config {
	pub frame_count: u32,
	pub render_size: (u32, u32),
	pub fractal:     Fractal,
	pub inverse:     bool,
	pub julia:       bool,

	pub start: Keyframe,
	pub stop:  Keyframe,

	pub palette: Palette,

	pub output_directory: PathBuf,
}

impl Config {
	/// Tries to validate `self`.
	///
	/// # Errors
	///
	/// If any configuration field couldn't be validated, an `Err` object is returned.
	pub fn validate(&self) -> Result<(), Error> {
		self.start.validate()?;
		self.stop.validate()?;

		if self.start.frame > self.stop.frame {
			Err(
				Error::InvalidConfig {
					message: format!("start frame ({}) cannot be after stop frame ({})", self.start.frame, self.stop.frame),
				},
			)
		} else if self.stop.frame >= self.frame_count {
			Err(
				Error::InvalidConfig {
					message: format!("stop frame ({}) cannot be after last frame ({})", self.stop.frame, self.frame_count - 0x1),
				},
			)
		} else {
			Ok(())
		}
	}
}
