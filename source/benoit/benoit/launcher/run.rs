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
use crate::benoit::launcher::{Launcher, Mode};
use crate::benoit::script::Script;

impl Launcher {
	#[must_use]
	pub fn run(self) -> Result<(), String> {
		Launcher::print_message();

		let mode = self.parse_arguments()?;

		let thread_count = match &mode {
			Mode::Script(configuration) => configuration.thread_count,
			_                           => 0x0,
		};

		self.setup(thread_count);

		return match mode {
			Mode::App(width, height) => {
				eprintln!("running in iteractive mode");

				let app = App::new(width, height);
				app.run()
			},
			Mode::Script(configuration) => {
				eprintln!("running in script mode");

				let script = Script::configure(configuration);
				script.run()
			},
		};
	}
}
