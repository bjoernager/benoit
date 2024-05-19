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

use crate::set_title;
use crate::error::Error;
use crate::instance::Instance;
use crate::config::Config;
use crate::keyframe::Keyframe;

use benoit::log;
use benoit::render::{Render, RenderConfig};
use benoit::stopwatch::Stopwatch;
use std::path::Path;

#[cfg(feature = "notify")]
use notify_rust::{Notification, Timeout};

impl Instance {
	/// Consumes the programme instance and starts execution.
	///
	/// # Errors
	///
	/// Returns any error that  occurs during executiom, if any at all.
	///
	/// This does not include panics, which are *not* handled by us (at the moment).

	#[allow(clippy::unused_self)]
	pub fn run(self, config: &Config) -> Result<(), Error> {
		let mut render = Render::new(config.render_size.0, config.render_size.1)?;

		let mut dump_frame = |keyframe: &Keyframe, path: &Path| -> Result<(), Error> {
			let render_config = RenderConfig {
				fractal: config.fractal.clone(),
				inverse: config.inverse,
				julia:   config.julia,

				max_iter_count: keyframe.max_iter_count,
				centre:         keyframe.centre.clone(),
				seed:           keyframe.seed.clone(),
				zoom:           keyframe.zoom.clone(),
			};

			run_job("render", || render.generate(&render_config));
			run_job("paint",  || render.plot(config.palette, keyframe.colour_range))?;
			run_job("dump",   || render.dump_image(path))?;

			Ok(())
		};

		let mut global_timer = Stopwatch::from_now();

		for keyframe in config.start.interpolate_between(&config.stop, config.frame_count) {
			log!(value, keyframe);

			let path = {
				let mut path = config.output_directory.clone();
				path.push(format!("frame{:010}.png", keyframe.frame));

				path
			};

			eprint!("\u{001B}[1m");
			eprintln!("frame no. {:.>10}       \u{2510}", keyframe.frame);
			eprintln!("       of {:.>10} total \u{2534}> \"{}\"", config.stop.frame, path.display());
			eprint!("\u{001B}[22m");

			let frame_timer = Stopwatch::test_result(|| dump_frame(&keyframe, &path))?;
			global_timer.note();

			eprintln!("                                        = \u{001B}[4m{frame_timer}\u{001B}[24m\u{001B}[2m ({global_timer} total)\u{001B}[22m");
			eprintln!();
		}

		global_timer.note();

		eprintln!("\u{001B}[1manimation completed\u{001B}[22m - took {global_timer} in total");
		set_title!("done");

		#[cfg(feature = "notify")]
		{
			// We don't really care if the notification fails.
			let _ = Notification::new()
				.appname("Benoit")
				.summary("\u{2728} Animation completed! \u{2728}")
				.body(&format!("Took {global_timer}."))
				.icon("benoit")
				.timeout(Timeout::Never)
				.show();
		}

		Ok(())
	}
}

pub fn run_job<F: FnOnce() -> Output, Output>(name: &str, func: F) -> Output {
	eprint!("\u{2610} running job `{name}` ...");
	set_title!("{name}");

	let (timer, result) = Stopwatch::test(func);

	// Indent to the 40th column; two have already been
	// used for the bullet:
	eprintln!("\r\u{2612} \u{001B}[2m{: <38}\u{001B}[22m\u{001B}[1m+ {timer}\u{001B}[22m", format!("completed job `{name}`"));

	result
}

//
//   ## ##
//  #  #  #
// #       #
// #       #
//  #     #
//   #   #
//    # #
//     #
//
