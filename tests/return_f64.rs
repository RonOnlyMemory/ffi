use ffi::*;

extern "win64" fn win64_function() -> f64 { 42.0 }
extern "sysv64" fn sysv64_function() -> f64 { 42.0 }

#[test]
fn return_f64() {
	unsafe {
		assert_eq!(win64::call(win64_function as _, &[]).f64(), 42.0);
		assert_eq!(sysv64::call(sysv64_function as _, &[]).f64(), 42.0);
	}
}
