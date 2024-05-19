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

macro_rules! use_mod {
	($visibility:vis, $name:ident) => {
		mod $name;
		$visibility use $name::*;
	};
}
pub(in crate) use use_mod;

pub mod config;
pub mod error;
pub mod instance;
pub mod keyframe;

use benoit::log;

#[cfg(windows)]
use windows::Win32::System::Console::SetConsoleTitleA;

/// Execute according to the build mode.
///
/// Use this as a shorthand for testing `debug_assertions`.
#[macro_export]
macro_rules! debug {
	($($body:stmt)*) => { if cfg!(debug_assertions) { $($body)? } };
}

/// Sets the terminal's title.
#[macro_export]
macro_rules! set_title {
	($($format:tt)*) => {{
		#[cfg(unix)]
		let set_title = |title: &str| {
			eprint!("\u{001B}]0;{title}\u{0007}");
		};

		#[cfg(windows)]
		let set_title = |title: &str| {
			unsafe { SetConsoleTitleA(title) };
		};

		let message = format!($($format)?);
		set_title(&message);
	}};
}

fn main() -> Result<(), i32> {
	let config = match config::Config::load() {
		Ok(config) => config,

		Err(message) => {
			log!(error, "{message}");
			return Err(0x2);
		},
	};

	log!(value, config);

	let instance: instance::Instance = Default::default();

	if let Err(message) = instance.run(&config) {
		log!(error, "{message}");
		return Err(0x1);
	}

	Ok(())
}
