use ffi::*;

extern "win64" fn win64_function(_: i8, _: i8, _: i8, _: i8, e: i8) {
	assert_eq!(e, -42);
}

extern "sysv64" fn sysv64_function(_: i8, _: i8, _: i8, _: i8, e: i8) {
	assert_eq!(e, -42);
}

#[test]
fn test_i8_stack() {
	unsafe {
		win64::call(win64_function as _, &[
			(-1_i8).into(),
			(-2_i8).into(),
			(-3_i8).into(),
			(-4_i8).into(),
			(-42_i8).into(),
		]);
		sysv64::call(sysv64_function as _, &[
			(-1_i8).into(),
			(-2_i8).into(),
			(-3_i8).into(),
			(-4_i8).into(),
			(-42_i8).into(),
		]);
	}
}
