use std::{arch::{global_asm, asm}, ffi::*};

use crate::{arg::Arg, return_value::ReturnValue};

global_asm!(r#"
.globl ffi_invoke_win64
ffi_invoke_win64:
	push rbp
	push r12
		mov rbp, rsp

		mov rax, [rsp +0x38]
		mov r12, [rsp +0x40]
		mov r10, r12
		mov r11, [rsp +0x48]

		2:
		cmp r10, 0
		jle 0f
			dec r10
			push [r11]
			add r11, 0x8
			jmp 2b
		0:

		sub rsp, 0x20
			call rax
		add rsp, 0x20
		
		2:
		cmp r12, 0
		jle 0f
			dec r12
			pop r11
			jmp 2b
		0:

	pop r12
	pop rbp
	ret
"#);

extern "win64" {
	// len must be even for the stack to be 16 bytes aligned
	pub fn ffi_invoke_win64(
		rcx: u64, rdx: u64, r8: u64, r9: u64,
		f: *const (),
		len: u64, rest: *const u64,
	) -> u64;
}

unsafe extern "win64" fn set_xmm_reg(
	_xmm0: f64, _xmm1: f64, _xmm2: f64, _xmm3: f64,
	rcx: u64, rdx: u64, r8: u64, r9: u64,
	f: *const (),
	len: u64, rest: *const u64,
) -> u64 {
	ffi_invoke_win64(rcx, rdx, r8, r9, f, len, rest)
}

pub unsafe fn call(f: *const (), args: &[Arg]) -> ReturnValue {
	let mut iter = args.into_iter();

	let a = iter.next();
	let rcx = a.map(|a| a.int()).unwrap_or(0);
	let xmm0 = a.map(|a| a.double()).unwrap_or(0.0);

	let a = iter.next();
	let rdx = a.map(|a| a.int()).unwrap_or(0);
	let xmm1 = a.map(|a| a.double()).unwrap_or(0.0);

	let a = iter.next();
	let r8 = a.map(|a| a.int()).unwrap_or(0);
	let xmm2 = a.map(|a| a.double()).unwrap_or(0.0);

	let a = iter.next();
	let r9 = a.map(|a| a.int()).unwrap_or(0);
	let xmm3 = a.map(|a| a.double()).unwrap_or(0.0);

	let iter = iter.map(|a| a.data());

	let mut args: Vec<u64> = iter.collect();
	if args.len()%2 == 0 {
		args.push(0);
	}
	let args: Vec<u64> = args.into_iter().rev().collect();
	let int = set_xmm_reg(
		xmm0, xmm1, xmm2, xmm3,
		rcx, rdx, r8, r9,
		f,
		args.len() as _, args.as_ptr(),
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
pub unsafe extern "C" fn ffi_call_win64(
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
