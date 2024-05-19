/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

/// The result of a render generation.
///
/// The *raw* buffer of the [`Render`](crate::render::Render) type uses this structure.
#[derive(Clone, Copy, Debug)]
#[repr(align(0x80))]
pub struct RawElement {
	/// The ammount of iterations made before escaping.
	pub iter_count: u64,

	/// The distance to the origin.
	pub distance: f64,
}

impl Default for RawElement {
	#[inline(always)]
	fn default() -> Self { Self { iter_count: 0x0, distance: 0.0 } }
}
