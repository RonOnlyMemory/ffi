use ffi::*;

extern "win64" fn win64_function(a: f32) {
	assert_eq!(a, 42.0);
}

extern "sysv64" fn sysv64_function(a: f32) {
	assert_eq!(a, 42.0);
}

#[test]
fn test_f32_reg() {
	unsafe {
		win64::call(win64_function as _, &[
			42_f32.into(),
		]);
	}
	unsafe {
		sysv64::call(sysv64_function as _, &[
			42_f32.into(),
		]);
	}
}
