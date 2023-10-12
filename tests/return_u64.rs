use ffi::*;

extern "win64" fn win64_function() -> u64 { 42 }
extern "sysv64" fn sysv64_function() -> u64 { 42 }

#[test]
fn return_u64() {
	unsafe {
		assert_eq!(win64::call(win64_function as _, &[]).u64(), 42);
		assert_eq!(sysv64::call(sysv64_function as _, &[]).u64(), 42);
	}
}
