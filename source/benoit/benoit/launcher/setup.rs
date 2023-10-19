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

use crate::benoit::VERSION;
use crate::benoit::launcher::Launcher;
use crate::benoit::palette::fill_palettes;

extern crate rayon;

use rayon::ThreadPoolBuilder;

impl Launcher {
	pub(super) fn setup(&self, thread_count: u32) {
		Launcher::set_title(format!("BENO\u{CE}T {:X}.{:X}.{:X}", VERSION.0, VERSION.1, VERSION.2).as_str());

		fill_palettes();

		eprintln!("using {} threads", thread_count);
		ThreadPoolBuilder::new().num_threads(thread_count as usize).build_global().unwrap();
	}
}
