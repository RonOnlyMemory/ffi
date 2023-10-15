


#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Arg {
	Int(u64),
	Double(f64),
}

impl Arg {
	pub fn int(self) -> u64 {
		match self {
			Self::Int(a) => a,
			Self::Double(a) => a as _,
		}
	}
	pub fn double(self) -> f64 {
		match self {
			Self::Int(a) => a as _,
			Self::Double(a) => a as _,
		}
	}
	pub fn data(self) -> u64 {
		match self {
			Self::Int(a) => a,
			Self::Double(a) => u64::from_ne_bytes(a.to_ne_bytes()),
		}
	}
}

impl From<u64> for Arg {
	fn from(value: u64) -> Self {
		Self::Int(value)
	}
}

impl From<u32> for Arg {
	fn from(value: u32) -> Self {
		Self::Int(value as _)
	}
}

impl From<u16> for Arg {
	fn from(value: u16) -> Self {
		Self::Int(value as _)
	}
}

impl From<u8> for Arg {
	fn from(value: u8) -> Self {
		Self::Int(value as _)
	}
}

impl From<i64> for Arg {
	fn from(value: i64) -> Self {
		Self::Int(u64::from_ne_bytes(value.to_ne_bytes()))
	}
}

impl From<i32> for Arg {
	fn from(value: i32) -> Self {
		let [a, b, c, d] = value.to_ne_bytes();
		Self::Int(u64::from_ne_bytes([a, b, c, d, 0, 0, 0, 0]))
	}
}

impl From<i16> for Arg {
	fn from(value: i16) -> Self {
		let [a, b] = value.to_ne_bytes();
		Self::Int(u64::from_ne_bytes([a, b, 0, 0, 0, 0, 0, 0]))
	}
}

impl From<i8> for Arg {
	fn from(value: i8) -> Self {
		let [a] = value.to_ne_bytes();
		Self::Int(u64::from_ne_bytes([a, 0, 0, 0, 0, 0, 0, 0]))
	}
}

impl From<f64> for Arg {
	fn from(value: f64) -> Self {
		Self::Double(value)
	}
}

impl From<f32> for Arg {
	fn from(value: f32) -> Self {
		let [a, b, c, d] = value.to_ne_bytes();
		Arg::Double(f64::from_ne_bytes([a, b, c, d, 0, 0, 0, 0]))
	}
}

impl<T> From<&T> for Arg {
	fn from(value: &T) -> Self {
		Self::Int(u64::from_ne_bytes((value as *const T as u64).to_ne_bytes()))
	}
}

impl<T> From<&mut T> for Arg {
	fn from(value: &mut T) -> Self {
		Self::Int(u64::from_ne_bytes((value as *mut T as u64).to_ne_bytes()))
	}
}

impl<T> From<*const T> for Arg {
	fn from(value: *const T) -> Self {
		Self::Int(u64::from_ne_bytes((value as u64).to_ne_bytes()))
	}
}

impl<T> From<*mut T> for Arg {
	fn from(value: *mut T) -> Self {
		Self::Int(u64::from_ne_bytes((value as u64).to_ne_bytes()))
	}
}
