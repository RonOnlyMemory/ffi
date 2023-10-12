use ffi::*;

extern "win64" fn win64_function(a: f32) {
	assert_eq!(a, 42.0);
}

#[test]
fn test_f32_reg() {
	unsafe {
		win64::call(win64_function as _, &[
			42_f32.into(),
		]);
	}
}
