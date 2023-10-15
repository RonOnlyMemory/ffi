use std::{arch::{global_asm, asm}, ffi::*};

use crate::{arg::Arg, return_value::ReturnValue};

global_asm!(include_str!("ffi_invoke_win64.s"));

extern "win64" {
	pub fn ffi_invoke_win64(
		rcx: u64, rdx: u64, r8: u64, r9: u64,
		f: *const (),
		len: u64, args_end: *const Arg,
	) -> u64;
}

unsafe extern "win64" fn set_xmm_reg(
	_xmm0: f64, _xmm1: f64, _xmm2: f64, _xmm3: f64,
	rcx: u64, rdx: u64, r8: u64, r9: u64,
	f: *const (),
	len: u64, args_end: *const Arg,
) -> u64 {
	ffi_invoke_win64(rcx, rdx, r8, r9, f, len, args_end)
}

pub unsafe fn call(f: *const (), args: &[Arg]) -> ReturnValue {
	assert_eq!(std::mem::size_of::<Arg>(), 0x10);

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

	let mut rest = 0 as *const Arg;
	let mut len = args.len();

	if len > 4 {
		rest = args[4..].as_ptr();
		len -= 4;
		rest = rest.offset(len as isize -1);
	} else {
		len = 0;
	}

	let int = set_xmm_reg(
		xmm0, xmm1, xmm2, xmm3,
		rcx, rdx, r8, r9,
		f,
		len as _, rest,
	);
	let mut double: f64;
	asm!(
		"movd {}, xmm0",
		out(reg) double,
	);
	ReturnValue::new(int.to_ne_bytes(), double.to_ne_bytes())
}

#[no_mangle]
pub unsafe extern "C" fn ffi_call_win64(
	f: *const c_void,
	len: c_ulonglong, args: *const (c_ulonglong, c_longlong),
	rt: c_int, ret: *mut c_longlong,
) -> c_int {
	assert_eq!(
		std::mem::size_of::<Arg>(),
		std::mem::size_of::<(c_ulonglong, c_longlong)>(),
	);
	for c in 0..len {
		let (c, _) = *args.offset(c as _);
		if c >= 2 {
			return 0;
		}
	}
	let args = std::mem::transmute(args);
	let rv = call(f as _, std::slice::from_raw_parts(args, len as _));
	match rt {
		0 => {
			*(ret as *mut u64) = rv.u64();
		}
		1 => {
			*(ret as *mut c_double) = rv.f64();
		}
		_ => {}
	};
	1
}
