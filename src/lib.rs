//! Famous "Fast inverse square root" algorithm implementation
//! along with obligatory original comments.

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(all(feature = "nightly",test))]
extern crate test;

use std::mem;
use std::f32;
use std::f64;

/// A trait that allows calculation of inverse square root into a Float(32).
pub trait InvSqrt32 {
	/// Calculates the inverse square root of a number
	fn inv_sqrt32(self) -> f32;
}

/// A trait that allows calculation of inverse square root into a Float(64).
pub trait InvSqrt64 {
	/// Calculates the inverse square root of a number.
	fn inv_sqrt64(self) -> f64;
}

/// Implementation of InvSqrt32 using the "Fast inverse square root" method.
/// This implementation somewhat provides look'n'feel of the original one.
/// If you want to uncomment 2nd iteration you actually have to make y mutable.
#[allow(non_upper_case_globals)]
impl InvSqrt32 for f32 {
	fn inv_sqrt32(self: f32) -> f32 {
		if cfg!(not(feature = "omit-checking")) {
			if self.is_sign_negative() {
				return f32::NAN;
			}
			else if !self.is_normal() {
				if self.is_nan() {
					return f32::NAN;
				}
				else if self.is_infinite() {
					return 0.0;
				}
				else {
					// Positive 0 et.al.
					return f32::INFINITY;
				}
			}
		}

		// Magic number based on Chris Lomont work:
		// const MAGIC_U32: u32 = 0x5f375a86;
		// The Original Magic Number:
		// const MAGIC_32: u32 = 0x5f3759df;
		const threehalfs: f32 = 1.5f32;
		let x2: f32 = self * 0.5f32;
		let mut i: u32 = unsafe { mem::transmute(self) }; // evil floating point bit level hacking
		i = 0x5f375a86 - (i >> 1);                        // what the fuck?
		let y: f32 = unsafe { mem::transmute(i) };
		let y  = y * ( threehalfs - ( x2 * y * y ) );     // 1st iteration
//		y  = y * ( threehalfs - ( x2 * y * y ) );       // 2nd iteration, this can be removed

		return y;
	}
}

/// Implementation of InvSqrt64 using the "Fast inverse square root" method.
/// This implementation provides somewhat more Rusty look'n'feel.
impl InvSqrt64 for f64 {
	fn inv_sqrt64(self: f64) -> f64 {
		if cfg!(not(feature = "omit-checking")) {
			if self.is_sign_negative() {
				return f64::NAN;
			}
			else if !self.is_normal() {
				if self.is_nan() {
					return f64::NAN;
				}
				else if self.is_infinite() {
					return 0.0;
				}
				else {
					// Positive 0 et.al.
					return f64::INFINITY;
				}
			}
		}

		// Magic number based on Chris Lomont work:
		const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
		const THREEHALFS: f64 = 1.5;
		let x2 = self * 0.5;
		let i = MAGIC_U64 - ( unsafe { mem::transmute::<_, u64>(self) } >> 1);
		let y: f64 = unsafe { mem::transmute(i) };

		y * ( THREEHALFS - ( x2 * y * y ) )
	}
}

impl InvSqrt32 for f64 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for i8 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for u8 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for i16 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for u16 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for i32 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for u32 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for i64 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for u64 {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for isize {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt32 for usize {
	fn inv_sqrt32(self) -> f32 {
		(self as f32).inv_sqrt32()
	}
}

impl InvSqrt64 for f32 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for i8 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for u8 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for i16 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for u16 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for i32 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for u32 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for i64 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for u64 {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for isize {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

impl InvSqrt64 for usize {
	fn inv_sqrt64(self) -> f64 {
		(self as f64).inv_sqrt64()
	}
}

#[cfg(test)]
mod test32 {
	use std::f32;
	use super::InvSqrt32;

	#[cfg(feature = "nightly")]
	use test::{black_box,Bencher};

	#[inline(always)]
	fn ref_inv_sqrt32(f: f32) -> f32 {
		1.0f32 / f.sqrt()
	}

	#[test]
	fn test_f32() {
		let value: f32 = 11.1111;
		assert!(value.inv_sqrt32().abs_sub(0.3f32) <= 0.000001);

		let mut value: f32 = 0.0f32;
		while value < 100.4f32 {
			assert!(value.inv_sqrt32().abs_sub(ref_inv_sqrt32(value)) <= 0.000001);
			value += 0.05f32;
		}
	}

	#[test]
	fn test_zero() {
		let zero = 0.0f32;
		assert_eq!(zero.inv_sqrt32(), f32::INFINITY);
	}

	#[test]
	fn test_negative() {
		let negative = -1.0f32;
		assert!(negative.inv_sqrt32().is_nan());
	}

	#[test]
	fn test_i8() {
		let value: i8 = 55;
		assert!(value.inv_sqrt32().abs_sub(0.1348399725f32) <= 0.000001);
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_plain_ref(b: &mut Bencher) {
		b.iter(|| {
			let f = black_box(1.2345f32);
			ref_inv_sqrt32(f)
		});
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_plain_impl(b: &mut Bencher) {
		b.iter(|| {
			let f = black_box(1.2345f32);
			f.inv_sqrt32()
		});
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_real_ref(b: &mut Bencher) {
		b.iter(|| {
			let v = black_box(2.3456f32);
			let f = black_box(1.2345f32);
			v / f.sqrt()
		});
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_real_impl(b: &mut Bencher) {
		b.iter(|| {
			let v = black_box(2.3456f32);
			let f = black_box(1.2345f32);
			v * f.inv_sqrt32()
		});
	}
}

#[cfg(test)]
mod test64 {
	use std::f64;
	use super::InvSqrt64;

	#[cfg(feature = "nightly")]
	use test::{black_box,Bencher};

	#[inline(always)]
	fn ref_inv_sqrt64(f: f64) -> f64 {
		1.0f64/f.sqrt()
	}

	#[test]
	fn test_f64() {
		let value: f64 = 11.1111;
		assert!(value.inv_sqrt64().abs_sub(0.3f64) <= 0.000001);

		let mut value: f64 = 0.0f64;
		while value < 100.4f64 {
			assert!(value.inv_sqrt64().abs_sub(ref_inv_sqrt64(value)) <= 0.000001);
			value += 0.05f64;
		}
	}

	#[test]
	fn test_zero() {
		let zero = 0.0f64;
		assert_eq!(zero.inv_sqrt64(), f64::INFINITY);
	}

	#[test]
	fn test_negative() {
		let negative = -1.0f64;
		assert!(negative.inv_sqrt64().is_nan());
	}

	#[test]
	fn test_i8() {
		let value: i8 = 55;
		assert!(value.inv_sqrt64().abs_sub(0.1348399725f64) <= 0.000001);
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_plain_ref(b: &mut Bencher) {
		b.iter(|| {
			let f = black_box(1.2345f64);
			ref_inv_sqrt64(f)
		});
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_plain_impl(b: &mut Bencher) {
		b.iter(|| {
			let f = black_box(1.2345f64);
			f.inv_sqrt64()
		});
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_real_ref(b: &mut Bencher) {
		b.iter(|| {
			let v = black_box(2.3456f64);
			let f = black_box(1.2345f64);
			v / f.sqrt()
		});
	}

	#[cfg(feature = "nightly")]
	#[bench]
	fn bench_real_impl(b: &mut Bencher) {
		b.iter(|| {
			let v = black_box(2.3456f64);
			let f = black_box(1.2345f64);
			v * f.inv_sqrt64()
		});
	}
}