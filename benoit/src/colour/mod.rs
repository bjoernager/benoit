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

//! Colour manipulations facilities.
//!
//! This includes the [`Colour`] structure for handling colours.
//!
//! These are primarily intended for colouring (plotting) renders, and is as such used by the `Palette` and `Render` types.

mod web;

use crate::error::Error;

#[cfg(feature = "wgpu-colour")]
use wgpu::Color as WGpuColour;

/// Represents a colour value.
///
/// Internally, this is encoded with RGBA parameters in linear gamma.
/// The type is, however, to be thought of as a purely abstract descriptor.
#[derive(Copy, Clone, Debug)]
pub struct Colour {
	red:   f32,
	green: f32,
	blue:  f32,
	alpha: f32,
}

/// Validates that the given parameter is valid.
///
/// That is, it must not be NaN.
///
/// In theory, it would be okay for parameters to be outside of the range 0..1, but for simplicity's sake this is avoided (at least in most cases).
///
/// If the parameter is infinite, no guarantees are made on our end for the results of conversion functions.
///
/// # Errors
///
/// If the value is NaN, an error message is returned in an [`Err`] object.
macro_rules! validate_parameter {
	($param:ident: $name:expr) => {
		#[allow(unused_mut)]
		let mut $param: f64 = $param.into();

		if $param.is_nan() {
			return Err(Error::NanColourParameter { name: $name });
		}
	};
}

impl Colour {
	/// Zips the provided parameters in a [`Colour`] structure.
	///
	/// # Errors
	///
	/// If any parameter is NaN, an error is returned.
	#[inline(always)]
	pub fn from_rgba<T: Into<f64>>(red: T, green: T, blue: T, alpha: T) -> Result<Self, Error> {
		validate_parameter!(red:   "red");
		validate_parameter!(green: "green");
		validate_parameter!(blue:  "blue");
		validate_parameter!(alpha: "alpha");

		Ok(Self {
			red:   red as f32,
			green: green as f32,
			blue:  blue as f32,
			alpha: alpha as f32,
		})
	}

	/// Zips the provided parameters in a [`Colour`] structure.
	///
	/// The values are interpreted as being sRGBA and.
	///
	/// # Errors
	///
	/// If any parameter is NaN, an error is returned.
	pub fn from_srgba<T: Into<f64>>(red: T, green: T, blue: T, alpha: T) -> Result<Self, Error> {
		validate_parameter!(red:   "red");
		validate_parameter!(green: "green");
		validate_parameter!(blue:  "blue");
		validate_parameter!(alpha: "alpha");

		for value in [&mut red, &mut green, &mut blue, &mut alpha] {
			*value = if *value > 0.091_568_643 {
				((*value + 0.055) / 1.055).powf(2.4)
			} else {
				*value / 11.920
			};
		}

		Ok(Self {
			red:   red as f32,
			green: green as f32,
			blue:  blue as f32,
			alpha: alpha as f32,
		})
	}

	/// Zips the provided HSVA parameters in a [`Colour`] structure.
	///
	/// # Errors
	///
	/// If any parameter is NaN, an error is returned.
	pub fn from_hsva<T: Into<f64>>(hue: T, saturation: T, value: T, alpha: T) -> Result<Self, Error> {
		validate_parameter!(hue:        "hue");
		validate_parameter!(saturation: "saturation");
		validate_parameter!(value:      "value");
		validate_parameter!(alpha:      "alpha");

		if saturation <= 0.0 {
			let value = value.clamp(0.0, 1.0);

			Self::from_srgba(value, value, value, alpha)
		} else {
			let h = hue % 1.0 * 6.0;
			let s = saturation.clamp(0.0, 1.0);
			let v = value.clamp(0.0, 1.0);
			let a = alpha;

			let f = h % 1.0;
			let p = v * (1.0 - s);
			let q = v * (-s).mul_add(f, 1.0); // v * (1.0 - s * f)
			let t = v * (-s).mul_add(1.0 - f, 1.0); // v * (1.0 - s * (1.0 - f))

			match h.trunc() as u8 {
				0x0 => Self::from_srgba(v, t, p, a),
				0x1 => Self::from_srgba(q, v, p, a),
				0x2 => Self::from_srgba(p, v, t, a),
				0x3 => Self::from_srgba(p, q, v, a),
				0x4 => Self::from_srgba(t, p, v, a),
				0x5 => Self::from_srgba(v, p, q, a),
				_   => unreachable!(), // Because h is guaranteed to be in the interval 0..=5.
			}
		}
	}

	/// Zips the provided HSVA parameters in a [`Colour`] structure.
	///
	/// # Errors
	///
	/// If any parameter is NaN, an error is returned.
	pub fn from_xyza<T: Into<f64>>(x: T, y: T, z: T, alpha: T) -> Result<Self, Error> {
		validate_parameter!(x:     "x");
		validate_parameter!(y:     "y");
		validate_parameter!(z:     "z");
		validate_parameter!(alpha: "alpha");

		// Convert directly to RGBA.

		let m: [[f64; 0x3]; 0x3] = [
			[ 3.240_969_942, -1.537_383_178, -0.498_610_760],
			[-0.969_243_636,  1.875_967_502,  0.041_555_057],
			[ 0.055_630_080, -0.203_976_959,  1.056_971_514],
		];

		let r = m[0x0][0x0].mul_add(x, m[0x0][0x1].mul_add(y, m[0x0][0x2] * z));
		let g = m[0x1][0x0].mul_add(x, m[0x1][0x1].mul_add(y, m[0x1][0x2] * z));
		let b = m[0x2][0x0].mul_add(x, m[0x2][0x1].mul_add(y, m[0x2][0x2] * z));

		Self::from_rgba(r, g, b, alpha)
	}

	/// Zips the provided LABA parameters in a [`Colour`] structure.
	///
	/// # Errors
	///
	/// If any parameter is NaN, an error is returned.
	pub fn from_laba<T: Into<f64>>(lightness: T, astar: T, bstar: T, alpha: T) -> Result<Self, Error> {
		validate_parameter!(lightness: "lightness");
		validate_parameter!(astar:     "a*");
		validate_parameter!(bstar:     "b*");
		validate_parameter!(alpha:     "alpha");

		// Convert to XYZA.

		// astar: a*
		// bstar: b*

		let kappa:   f64 = 903.296_296_296;
		let epsilon: f64 = 0.008_856_452;

		let f1 = (lightness + 16.0) / 116.0;
		let f0 = astar.mul_add(1.0 / 500.0, f1);
		let f2 = (-bstar).mul_add(1.0 / 200.0, f1);

		let temporary = (lightness + 16.0) / 116.0;

		let mut x = f0 * f0 * f0;
		let mut y = temporary * temporary * temporary;
		let mut z = f2 * f2 * f2;

		if x         <= epsilon         { x = 0.964_016_736 * (f0.mul_add(116.0, -16.0) / kappa) };
		if lightness <= kappa * epsilon { y = lightness / kappa };
		if z         <= epsilon         { z = 0.825_104_603 * (f2.mul_add(116.0, -16.0) / kappa) };

		Self::from_xyza(x, y, z, alpha)
	}

	/// Zips the provided LCHA parameters in a [`Colour`] structure.
	///
	/// # Errors
	///
	/// If any parameter is NaN, an error is returned.
	pub fn from_lcha<T: Into<f64>>(lightness: T, chroma: T, hue:T, alpha: T) -> Result<Self, Error> {
		validate_parameter!(lightness: "lightness");
		validate_parameter!(chroma:    "chroma");
		validate_parameter!(hue:       "hua");
		validate_parameter!(alpha:     "alpha");

		// Convert to LABA.

		// Convert turns to radians:
		// 1 turn = 2pi radians

		hue *= std::f64::consts::PI * 2.0;

		let astar = chroma * hue.cos(); // a*
		let bstar = chroma * hue.sin(); // b*

		Self::from_laba(lightness, astar, bstar, alpha)
	}

	/// Transforms the contained parameters to sRGBA colour space.
	#[must_use]
	pub fn to_srgba(self) -> (f64, f64, f64, f64) {
		assert!(!self.red.is_nan());
		assert!(!self.green.is_nan());
		assert!(!self.blue.is_nan());
		assert!(!self.alpha.is_nan());

		let mut red   = f64::from(self.red);
		let mut green = f64::from(self.green);
		let mut blue  = f64::from(self.blue);
		let mut alpha = f64::from(self.alpha);

		for value in [&mut red, &mut green, &mut blue, &mut alpha] {
			*value = if *value > 0.000_313_080 {
				value.powf(0.416_666_667).mul_add(1.055, -0.055)
			} else {
				11.920 * *value
			};
		}

		(red, green, blue, alpha)
	}

	/// Transforms the contained parameters to integer sRGBA with 8 bits per channel.
	#[must_use]
	pub fn to_srgba8(self) -> (u8, u8, u8, u8) {
		assert!(!self.red.is_nan());
		assert!(!self.green.is_nan());
		assert!(!self.blue.is_nan());
		assert!(!self.alpha.is_nan());

		let (red, green, blue, alpha) = self.to_srgba();

		let red   = (red   * f64::from(u8::MAX)).trunc() as u8;
		let green = (green * f64::from(u8::MAX)).trunc() as u8;
		let blue  = (blue  * f64::from(u8::MAX)).trunc() as u8;
		let alpha = (alpha * f64::from(u8::MAX)).trunc() as u8;

		(red, green, blue, alpha)
	}

	/// Transforms the contained parameters to integer sRGBA with 16 bits per channel.
	#[must_use]
	pub fn to_srgba16(self) -> (u16, u16, u16, u16) {
		assert!(!self.red.is_nan());
		assert!(!self.green.is_nan());
		assert!(!self.blue.is_nan());
		assert!(!self.alpha.is_nan());

		let (red, green, blue, alpha) = self.to_srgba();

		let red   = (red   * f64::from(u16::MAX)).trunc() as u16;
		let green = (green * f64::from(u16::MAX)).trunc() as u16;
		let blue  = (blue  * f64::from(u16::MAX)).trunc() as u16;
		let alpha = (alpha * f64::from(u16::MAX)).trunc() as u16;

		(red, green, blue, alpha)
	}

	/// Transforms the contained parameters to integer sRGBA with 32 bits per channel.
	#[must_use]
	pub fn to_srgba32(self) -> (u32, u32, u32, u32) {
		assert!(!self.red.is_nan());
		assert!(!self.green.is_nan());
		assert!(!self.blue.is_nan());
		assert!(!self.alpha.is_nan());

		let (red, green, blue, alpha) = self.to_srgba();

		let red   = (red   * f64::from(u32::MAX)).trunc() as u32;
		let green = (green * f64::from(u32::MAX)).trunc() as u32;
		let blue  = (blue  * f64::from(u32::MAX)).trunc() as u32;
		let alpha = (alpha * f64::from(u32::MAX)).trunc() as u32;

		(red, green, blue, alpha)
	}
}

impl Default for Colour {
	#[inline(always)]
	fn default() -> Self { Self::from_rgba(1.0, 0.0, 1.0, 1.0).unwrap() }
}

// Will be used by benoit-gui:
#[cfg(feature = "wgpu-colour")]
impl From<Colour> for WGpuColour {
	/// Transforms the contained parameters to a [`WGpuColour`] object.
	///
	/// To be used by the `[benoit-gui]` crate.
	///
	/// Currently, this also transforms the colour value to sRGBA in the process.
	fn from(value: Colour) -> Self {
		// I think wgpu expects perceptual gamma?

		let (r, g, b, a) = value.to_srgba();
		Self { r, g, b, a}
	}
}
