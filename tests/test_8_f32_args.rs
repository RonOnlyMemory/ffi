use ffi::*;

extern "win64" fn win64_function(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) {
	assert_eq!(a, 1.0);
	assert_eq!(b, 2.0);
	assert_eq!(c, 3.0);
	assert_eq!(d, 4.0);
	assert_eq!(e, 5.0);
	assert_eq!(f, 6.0);
	assert_eq!(g, 7.0);
	assert_eq!(h, 8.0);
}

extern "sysv64" fn sysv64_function(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) {
	assert_eq!(a, 1.0);
	assert_eq!(b, 2.0);
	assert_eq!(c, 3.0);
	assert_eq!(d, 4.0);
	assert_eq!(e, 5.0);
	assert_eq!(f, 6.0);
	assert_eq!(g, 7.0);
	assert_eq!(h, 8.0);
}

#[test]
fn test_8_f32_args() {
	unsafe {
		win64::call(win64_function as _, &[
			1_f32.into(),
			2_f32.into(),
			3_f32.into(),
			4_f32.into(),
			5_f32.into(),
			6_f32.into(),
			7_f32.into(),
			8_f32.into(),
		]);
	}
	unsafe {
		sysv64::call(sysv64_function as _, &[
			1_f32.into(),
			2_f32.into(),
			3_f32.into(),
			4_f32.into(),
			5_f32.into(),
			6_f32.into(),
			7_f32.into(),
			8_f32.into(),
		]);
	}
}
