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

//! Timing utilities.
//!
//! These are used e.g. by `benoit-cli` for timing renders.

use std::fmt::{Display, Error, Formatter};
use std::time::{Duration, Instant};

/// Type for timing.
///
/// This primarily includes the timing of functions.
pub struct Stopwatch {
	start: Instant,
	stop:  Option<Instant>,
}

impl Stopwatch {
	/// Creates a new stopwatch, starting at the time of calling.
	#[must_use]
	pub fn from_now() -> Self {
		Self {
			start: Instant::now(),
			stop:  None,
		}
	}

	/// Times the execution time of `func`.
	///
	/// This is done by starting a stopwatch right before calling the function, followed by noting after it has returned.
	///
	/// The return value is passed on alongside the stopwatch.
	#[must_use]
	pub fn test<F: FnOnce() -> Output, Output>(func: F) -> (Self, Output) {
		let mut timer = Self::from_now();
		let result = func();
		timer.note();

		(timer, result)
	}

	/// Times the execution time of `func`.
	///
	/// This is done by starting a stopwatch right before calling the function, followed by noting after it has returned.
	///
	/// Differently from [`test`](Self::test), this method is suitable for functions which return a [`Result`] object with `T = ()`.
	///
	/// # Errors
	///
	/// Returns the error yielded by `func` (if any).
	pub fn test_result<F: FnOnce() -> Result<(), E>, E>(func: F) -> Result<Self, E> {
		let (timer, result) = Self::test(func);
		result.map(|()| timer)
	}

	/// Notes the current time for use with comparing the
	pub fn note(&mut self) {
		self.stop = Some(Instant::now());
	}

	/// Calculates the duration from start to the last note.
	///
	/// If no note has been made (as by using [`note`](Self::note)), a [`None`] instance is returned.
	#[must_use]
	pub fn duration(&self) -> Option<Duration> {
		let stop  = self.stop?;

		Some(stop.duration_since(self.start))
	}

	/// Calculates the total duration since the stopwatch' creation.
	#[must_use]
	pub fn total_duration(&self) -> Duration {
		self.start.elapsed()
	}
}

impl Display for Stopwatch {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let duration = if let Some(duration) = self.duration() {
			duration.as_nanos() as f64
		} else {
			return Err(Error);
		};

		macro_rules! test_unit {
			($length:expr, $symbol:literal) => {{
				if duration >= $length {
					write!(f, "{:.3}{}", duration / $length, $symbol)?;
					return Ok(());
				}
			}};
		}

		test_unit!(31_557_600_000_000_000_000_000_000.0, "Ma");        // billion (julian) years
		test_unit!(31_557_600_000_000_000_000_000.0,     "Ma");        // million (julian) years
		test_unit!(31_557_600_000_000_000_000.0,         "ka");        // thousand (julian) years
		test_unit!(31_557_600_000_000_000.0,             "a");         // (julian) year
		test_unit!(86_400_000_000_000.0,                 "d");         // day
		test_unit!(3_600_000_000_000.0,                  "h");         // hour
		test_unit!(60_000_000_000.0,                     "min");       // minute
		test_unit!(1_000_000_000.0,                      "s");         // second
		test_unit!(1_000_000.0,                          "ms");        // millisecond
		test_unit!(1_000.0,                              "\u{03BC}s"); // microsecond
		test_unit!(1.0,                                  "ns");        // nanosecond?

		unreachable!();
	}
}
