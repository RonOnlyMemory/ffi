



pub struct ReturnValue {
	int: [u8; 8],
	double: [u8; 8],
}

impl ReturnValue {
	pub fn new(int: [u8; 8], double: [u8; 8]) -> Self {
		Self {
			int,
			double,
		}
	}
	pub fn u64(&self) -> u64 {
		u64::from_ne_bytes(self.int)
	}
	pub fn u32(&self) -> u32 {
		let [a, b, c, d, _, _, _, _] = self.int;
		u32::from_ne_bytes([a, b, c, d])
	}
	pub fn u16(&self) -> u16 {
		let [a, b, _, _, _, _, _, _] = self.int;
		u16::from_ne_bytes([a, b])
	}
	pub fn u8(&self) -> u8 {
		let [a, _, _, _, _, _, _, _] = self.int;
		u8::from_ne_bytes([a])
	}
	pub fn i64(&self) -> i64 {
		i64::from_ne_bytes(self.int)
	}
	pub fn i32(&self) -> i32 {
		let [a, b, c, d, _, _, _, _] = self.int;
		i32::from_ne_bytes([a, b, c, d])
	}
	pub fn i16(&self) -> i16 {
		let [a, b, _, _, _, _, _, _] = self.int;
		i16::from_ne_bytes([a, b])
	}
	pub fn i8(&self) -> i8 {
		let [a, _, _, _, _, _, _, _] = self.int;
		i8::from_ne_bytes([a])
	}
	pub fn f64(&self) -> f64 {
		f64::from_ne_bytes(self.double)
	}
	pub fn f32(&self) -> f32 {
		let [a, b, c, d, _, _, _, _] = self.double;
		f32::from_ne_bytes([a, b, c, d])
	}
}
