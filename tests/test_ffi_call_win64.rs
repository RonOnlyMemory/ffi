use ffi::win64::ffi_call_win64;
use std::mem::transmute;

extern "win64" fn win64_function(a: u64, b: u64, c: u64, d: u64, e: u64) {
	assert_eq!(a, 1);
	assert_eq!(b, 2);
	assert_eq!(c, 3);
	assert_eq!(d, 4);
	assert_eq!(e, 5);
}

#[test]
fn test_ffi_call_win64() {
	unsafe {
		let args = [
			(0_u64, transmute::<_, i64>(1_u64)),
			(0_u64, transmute(2_u64)),
			(0_u64, transmute(3_u64)),
			(0_u64, transmute(4_u64)),
			(0_u64, transmute(5_u64)),
		];
		let mut ret: u64 = 0;
		ffi_call_win64(
			win64_function as _,
			args.len() as _,
			args.as_ptr(),
			0,
			(&mut ret) as *mut _ as *mut _,
		);
	}
}
