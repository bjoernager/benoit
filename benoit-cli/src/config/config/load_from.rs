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

	benoit-cli is distributed in (the) hope that it
	will be useful, but WITHOUT ANY WARRANTY; with-
	out even the implied warranty of MERCHANTABILITY
	or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU Gene-
	ral Public License along with benoit-cli. If
	not, see <https://www.gnu.org/licenses/>.
*/

use crate::config::{Config, Section, Take};
use crate::error::Error;
use crate::keyframe::Keyframe;

use std::path::Path;

macro_rules! take_field {
	($name:ident in ($section:expr)) => {{
		$section.get_child(stringify!($name)).take()?
	}};

	($name:ident in ($section:expr) str) => {{
		$section.get_child(stringify!($name)).take_from_str()??
	}};
}

impl Config {
	/// Loads the configuration file at `path`.
	///
	/// # Errors
	///
	/// If the configuration file could not be read or parsed, an error message is returned.
	pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
		let root_section = Section::create_root(path)?;

		let render_section: Section = root_section.get_child("render").take()?;
		let final_section:  Section = root_section.get_child("final").take()?;
		let output_section: Section = root_section.get_child("output").take()?;

		let render_start_section: Section = render_section.get_child("start").take()?;
		let render_stop_section:  Section = render_section.get_child("stop").take()?;

		let render_width  = take_field!(width  in (render_section));
		let render_height = take_field!(height in (render_section));

		Ok(Self {
			frame_count: take_field!(count   in (render_section)),
			render_size: (render_width, render_height),
			fractal:     take_field!(fractal in (render_section) str),
			inverse:     take_field!(inverse in (render_section)),
			julia:       take_field!(julia   in (render_section)),

			start: Keyframe {
				frame:          take_field!(frame          in (render_start_section)),
				max_iter_count: take_field!(max_iter_count in (render_start_section)),
				centre:         take_field!(centre         in (render_start_section)),
				seed:           take_field!(seed           in (render_start_section)),
				zoom:           take_field!(zoom           in (render_start_section)),
				colour_range:   take_field!(colour_range   in (render_start_section)),
			},
			stop: Keyframe {
				frame:          take_field!(frame          in (render_stop_section)),
				max_iter_count: take_field!(max_iter_count in (render_stop_section)),
				centre:         take_field!(centre         in (render_stop_section)),
				seed:           take_field!(seed           in (render_stop_section)),
				zoom:           take_field!(zoom           in (render_stop_section)),
				colour_range:   take_field!(colour_range   in (render_stop_section)),
			},

			palette: take_field!(palette in (final_section) str),

			output_directory: take_field!(directory in (output_section)),
		})
	}
}
