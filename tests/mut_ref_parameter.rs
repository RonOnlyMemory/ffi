use ffi::*;

struct A {
	a: u64,
}

extern "win64" fn win64_function(a: &mut A) {
	a.a = 42;
}

extern "sysv64" fn sysv64_function(a: &mut A) {
	a.a = 42;
}

#[test]
fn mut_ref_parameter() {
	unsafe {
		let mut a = A { a: 32 };
		win64::call(win64_function as _, &[
			(&mut a).into()
		]);
		assert_eq!(a.a, 42);
	}
	unsafe {
		let mut a = A { a: 32 };
		sysv64::call(sysv64_function as _, &[
			(&mut a).into()
		]);
		assert_eq!(a.a, 42);
	}
}
