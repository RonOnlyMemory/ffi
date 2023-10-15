use ffi::*;

extern "win64" fn win64_function(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) {
	assert_eq!(a, 1);
	assert_eq!(b, 2);
	assert_eq!(c, 3);
	assert_eq!(d, 4);
	assert_eq!(e, 5);
	assert_eq!(f, 6);
}

extern "sysv64" fn sysv64_function(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) {
	assert_eq!(a, 1);
	assert_eq!(b, 2);
	assert_eq!(c, 3);
	assert_eq!(d, 4);
	assert_eq!(e, 5);
	assert_eq!(f, 6);
}

#[test]
fn test_6_args() {
	unsafe {
		win64::call(win64_function as _, &[
			1.into(),
			2.into(),
			3.into(),
			4.into(),
			5.into(),
			6.into(),
		]);
	}
	unsafe {
		sysv64::call(sysv64_function as _, &[
			1.into(),
			2.into(),
			3.into(),
			4.into(),
			5.into(),
			6.into(),
		]);
	}
}
