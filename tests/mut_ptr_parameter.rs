use ffi::*;

struct A {
	a: u64,
}

unsafe extern "win64" fn win64_function(a: *mut A) {
	(*a).a = 42;
}

#[test]
fn mut_ptr_parameter() {
	unsafe {
		let mut a = A { a: 32 };
		win64::call(win64_function as _, &[
			(&mut a as *mut A).into()
		]);
		assert_eq!(a.a, 42);
	}
}
