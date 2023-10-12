use ffi::*;

extern "win64" fn win64_function(a: u64, b: u64, c: u64, d: u64, e: u64) {
	assert_eq!(a, 1);
	assert_eq!(b, 2);
	assert_eq!(c, 3);
	assert_eq!(d, 4);
	assert_eq!(e, 5);
}

#[test]
fn test_5_args() {
	unsafe {
		win64::call(win64_function as _, &[
			1.into(),
			2.into(),
			3.into(),
			4.into(),
			5.into(),
		]);
	}
}
