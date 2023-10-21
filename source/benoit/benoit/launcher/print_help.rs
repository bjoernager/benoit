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

use crate::benoit::launcher::Launcher;

use std::process::exit;

impl Launcher {
	pub(super) fn print_help() -> ! {
		println!("Usage:");
		println!("    benoit [--help] [path]");
		println!();
		println!("Configuration:");
		println!("    thread_count               0..=4294967295");
		println!();
		println!("    fractal                    \"mandelbrot\"|\"multibrot3\"|\"multibrot4\"|\"burningship\"|\"tricorn\"");
		println!("    inverse                    false|true");
		println!("    julia                      false|true");
		println!();
		println!("    canvas_width               2..=4294967295");
		println!("    canvas_height              2..=4294967295");
		println!("    start_frame                0..=4294967295");
		println!("    stop_frame                 start_frame..=4294967295");
		println!("    palette                    \"emerald\"|\"fire\"|\"greyscale\"|\"hsv\"|\"lch\"|\"ruby\"|\"sapphire\"|\"simple\"|\"twilight\"");
		println!();
		println!("    dump_path");
		println!("    image_format               \"png\"|\"webp\"");
		println!();
		println!("Additionally, the following parameters may be set in the \"start\" and \"stop\" sections:");
		println!();
		println!("    real                       \"0.0\"\u{B1}");
		println!("    imaginary                  \"0.0\"\u{B1}");
		println!("    extra_real                 \"0.0\"\u{B1}");
		println!("    extra_imaginary            \"0.0\"\u{B1}");
		println!("    zoom                       \"0.0\"<");
		println!("    maximum_iteration_count    1..=4294967295");
		println!("    colour_range               2.0+");
		println!();
		println!("If not animating (stop_frame equals zero), only the latter section is used.");
		println!("Note that real, imaginary, extra_real, extra_imaginary, and zoom - if present - must be quoted floats.");
		println!("When choosing image format, keep in mind that PNG is faster whilst WebP yields better compression. Both are, however, lossless.");
		println!();

		exit(0x0);
	}
}
