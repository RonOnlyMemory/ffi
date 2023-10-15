use std::{arch::{global_asm, asm}, ffi::*};

use crate::{arg::Arg, return_value::ReturnValue};

global_asm!(r#"
.globl ffi_invoke_sysv64
ffi_invoke_sysv64:
	sub rsp, 0x80
	push rbp
	push r12
		mov rbp, rsp

		mov rax, [rsp +0x98]
		mov r12, [rsp +0xa0]
		mov r10, r12
		mov r11, [rsp +0xa8]

		test r12, 1
		jnz 4f
			push 0
		4:

		2:
		cmp r10, 0
		jle 0f
			dec r10
			push [r11]
			add r11, 0x8
			jmp 2b
		0:

		call rax

		test r12, 1
		jnz 4f
			pop r11
		4:

		2:
		cmp r12, 0
		jle 0f
			dec r12
			pop r11
			jmp 2b
		0:

	pop r12
	pop rbp
	add rsp, 0x80
	ret
"#);

extern "sysv64" {
	pub fn ffi_invoke_sysv64(
		rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9: u64,
		xmm0: f64, xmm1: f64, xmm2: f64, xmm3: f64, xmm4: f64, xmm5: f64, xmm6: f64, xmm7: f64,
		f: *const (),
		len: u64, rest: *const u64,
	) -> u64;
}

pub unsafe fn call(f: *const (), args: &[Arg]) -> ReturnValue {
	let mut regs = [Option::<u64>::None; 6];
	let mut xmms = [Option::<f64>::None; 8];

	let mut reg_index = 0;
	let mut xmm_index = 0;

	let mut rest = Vec::<u64>::new();

	for arg in args {
		match arg {
			Arg::Int(_) => {
				if reg_index < 6 {
					regs[reg_index] = Some(arg.int());
					reg_index += 1;
				} else {
					rest.push(arg.data());
				}
			}
			Arg::Double(_) => {
				if xmm_index < 8 {
					xmms[xmm_index] = Some(arg.double());
					xmm_index += 1;
				} else {
					rest.push(arg.data());
				}
			}
		}
	}

	let rest: Vec<u64> = rest.into_iter().rev().collect();
	let int = ffi_invoke_sysv64(
		regs[0].unwrap_or(0), regs[1].unwrap_or(0),
		regs[2].unwrap_or(0), regs[3].unwrap_or(0),
		regs[4].unwrap_or(0), regs[5].unwrap_or(0),
		xmms[0].unwrap_or(0.0), xmms[1].unwrap_or(0.0),
		xmms[2].unwrap_or(0.0), xmms[3].unwrap_or(0.0),
		xmms[4].unwrap_or(0.0), xmms[5].unwrap_or(0.0),
		xmms[6].unwrap_or(0.0), xmms[7].unwrap_or(0.0),
		f,
		rest.len() as _, rest.as_ptr(),
	);
	let mut double: f64;
	asm!(
		"movd {}, xmm0",
		out(reg) double,
	);
	ReturnValue::new(int.to_ne_bytes(), double.to_ne_bytes())
}

unsafe fn call_intern(f: *const (), len: u64, args: *const (u64, u64)) -> Option<ReturnValue> {
	let mut data = Vec::<Arg>::new();
	for c in 0..len as isize {
		let a = args.offset(c);
		let (t, d) = *a;
		match t {
			0 => {
				data.push(Arg::Int(d));
			}
			1 => {
				data.push(Arg::Double(f64::from_ne_bytes(d.to_ne_bytes())));
			}
			_ => None?,
		}
	}
	Some(call(f, &data))
}

#[no_mangle]
pub unsafe extern "C" fn ffi_call_sysv64(
	f: *const c_void,
	len: c_longlong, args: *const (c_longlong, c_longlong),
	rt: c_int, ret: *mut c_longlong,
) -> c_int {
	if let Some(rv) = call_intern(f as _, args as _, len as _) {
		match rt {
			0 => {
				*(ret as *mut u64) = rv.u64();
			}
			1 => {
				*(ret as *mut c_double) = rv.f64();
			}
			_ => return 0,
		}
		1
	} else {
		0
	}
}
