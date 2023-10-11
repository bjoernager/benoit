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

mod benoit;

use crate::benoit::VERSION;
use crate::benoit::app::App;
use crate::benoit::configuration::Configuration;
use crate::benoit::script::Script;

extern crate rayon;

use rayon::ThreadPoolBuilder;
use std::env::args;
use std::process::exit;
use std::thread::available_parallelism;

fn main() {
	println!();
	println!("\u{1B}[1mBENO\u{CE}T\u{1B}[0m {:X}.{:X}.{:X}", VERSION[0x0], VERSION[0x1], VERSION[0x2]);
	println!("Copyright 2021, 2023 Gabriel Bj\u{F8}rnager Jensen.");
	println!();
	println!("Le p\u{E8}re cogita et c'est pourquoi il fut.");
	println!();

	let mut arguments = args();

	let (mut configuration, interative) = match arguments.nth(0x1) {
		Some(path) => (Configuration::load(path.as_str()), false),
		_          => (Configuration::default(),           true),
	};

	configuration.thread_count = if configuration.thread_count == 0x0 {
		match available_parallelism() {
			Ok(ammount) => ammount.get() as u32,
			_           => 0x2, // We assume at least two threads.
		}
	} else {
		configuration.thread_count
	};

	eprintln!("using {} threads", configuration.thread_count);
	ThreadPoolBuilder::new().num_threads(configuration.thread_count as usize).build_global().unwrap();

	let code = if interative {
		eprintln!("running iteractive mode");

		let app = App::configure(configuration);
		app.run()
	} else {
		eprintln!("running script mode");

		let script = Script::configure(configuration);
		script.run()
	};

	exit(code);
}
