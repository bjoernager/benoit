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
use crate::benoit::launcher::Launcher;
use crate::benoit::script::Script;

use std::thread::available_parallelism;

impl Launcher {
	#[must_use]
	pub fn run(self) -> i32 {
		Launcher::print_message();

		let (mut configuration, interative) = self.parse_arguments();

		configuration.thread_count = if configuration.thread_count == 0x0 {
			match available_parallelism() {
				Ok(ammount) => ammount.get() as u32,
				_           => 0x2, // We assume at least two threads.
			}
		} else {
			configuration.thread_count
		};

		self.setup(configuration.thread_count);

		return if interative {
			eprintln!("running in iteractive mode");

			let app = App::configure(configuration);
			app.run()
		} else {
			eprintln!("running in script mode");

			let script = Script::configure(configuration);
			script.run()
		};
	}
}
