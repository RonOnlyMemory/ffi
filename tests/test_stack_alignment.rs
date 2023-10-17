use ffi::{*, arg::Arg};
use std::arch::global_asm;

global_asm!(r#"
.globl return_rsp
return_rsp:
	mov rax, rsp
	ret
"#);

extern "win64" {
	fn return_rsp() -> u64;
}

#[test]
fn test_stack_alignment() {
	let mut args = Vec::<Arg>::new();
	unsafe {
		assert_eq!(return_rsp()%16, 8); // sanity check
	}
	for c in 0..11 {
		unsafe {
			let ret = win64::call(return_rsp as _, &args);
			assert_eq!(ret.u64()%16, 8);
		}
		unsafe {
			let ret = sysv64::call(return_rsp as _, &args);
			assert_eq!(ret.u64()%16, 8);
		}
		args.push(c.into());
	}
}
