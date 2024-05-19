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

//! Runtime.

mod run;

use rug::float::{free_cache, FreeCache};

/// Denotes a programme instance.
///
/// In theory, due to Rust's safety, any ammount of instances can be created at any time... disregarding hardware constraints, of course.
///
/// The choice of creating this structure in the first place is a personal one.
/// I can be argued, however, that some things should be leveraged using [`Drop`].
pub struct Instance;

impl Default for Instance {
	#[inline(always)]
	fn default() -> Self { Self }
}

impl Drop for Instance {
	fn drop(&mut self) {
		free_cache(FreeCache::All);
	}
}
