use ffi::*;

extern "win64" fn win64_function(_: f32, _: f32, _: f32, _: f32, e: f32) {
	assert_eq!(e, 42.0);
}

extern "sysv64" fn sysv64_function(_: f32, _: f32, _: f32, _: f32, e: f32) {
	assert_eq!(e, 42.0);
}

#[test]
fn test_f32_stack() {
	unsafe {
		win64::call(win64_function as _, &[
			1.0_f32.into(),
			2.0_f32.into(),
			3.0_f32.into(),
			4.0_f32.into(),
			42_f32.into(),
		]);
	}
	unsafe {
		sysv64::call(sysv64_function as _, &[
			1.0_f32.into(),
			2.0_f32.into(),
			3.0_f32.into(),
			4.0_f32.into(),
			42_f32.into(),
		]);
	}
}
