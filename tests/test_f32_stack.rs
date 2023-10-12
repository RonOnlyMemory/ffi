use ffi::*;

extern "win64" fn win64_function(_: f32, _: f32, _: f32, _: f32, e: f32) {
	assert_eq!(e, 42.0);
}

#[test]
fn test_f32_stack() {
	unsafe {
		win64::call(win64_function as _, &[
			1.into(),
			2.into(),
			3.into(),
			4.into(),
			42_f32.into(),
		]);
	}
}
