use ffi::win64;



#[no_mangle]
extern "win64" fn f(a: u64, b: f64, c: f32, d: u8) -> f64 {
	assert_eq!(a, 1);
	assert_eq!(b, 2.0);
	assert_eq!(c, 3.0);
	assert_eq!(d, 4);
	42.0
}

fn main() {
	unsafe {
		let ret = win64::call(f as _, &[
			1.into(),
			2.0_f64.into(),
			3.0_f32.into(),
			4.into(),
		]).f64();
		assert_eq!(ret, 42.0);
	}
}
