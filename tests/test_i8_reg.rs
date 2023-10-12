use ffi::*;

extern "win64" fn win64_function(a: i8) {
	assert_eq!(a, -42);
}

#[test]
fn test_i8_reg() {
	unsafe {
		win64::call(win64_function as _, &[
			(-42_i8).into(),
		]);
	}
}
