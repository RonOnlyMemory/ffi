use ffi::*;

struct A {
	a: u64,
}

extern "win64" fn win64_function(a: &A) {
	assert_eq!(a.a, 42);
}

extern "sysv64" fn sysv64_function(a: &A) {
	assert_eq!(a.a, 42);
}

#[test]
fn ref_parameter() {
	unsafe {
		let a = A { a: 42 };
		win64::call(win64_function as _, &[
			(&a).into()
		]);
	}
	unsafe {
		let a = A { a: 42 };
		sysv64::call(sysv64_function as _, &[
			(&a).into()
		]);
	}
}
