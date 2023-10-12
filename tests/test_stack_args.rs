use ffi::*;

extern "sysv64" fn sysv64_function(_: u64, _: u64, _: u64, _: u64, _: u64, _: u64, a: u64, b: u64, c: u64) {
	assert_eq!(a, 42);
	assert_eq!(b, 43);
	assert_eq!(c, 44);
}

#[test]
fn test_stack_args() {
	unsafe {
		sysv64::call(sysv64_function as _, &[
			1.into(),
			2.into(),
			3.into(),
			4.into(),
			5.into(),
			6.into(),
			42.into(),
			43.into(),
			44.into(),
		]);
	}
}
