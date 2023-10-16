use core::{arch::{global_asm, asm}, ffi::*};

use crate::{arg::Arg, return_value::ReturnValue};

global_asm!(include_str!("ffi_invoke_sysv64.s"));

extern "sysv64" {
	pub fn ffi_invoke_sysv64(
		rdi: u64, rsi: u64, rdx: u64, rcx: u64, r8: u64, r9: u64,
		xmm0: f64, xmm1: f64, xmm2: f64, xmm3: f64, xmm4: f64, xmm5: f64, xmm6: f64, xmm7: f64,
		f: *const (),
		len: u64, rest: *const Arg,
	) -> u64;
}

pub unsafe fn call(f: *const (), args: &[Arg]) -> ReturnValue {
	let mut regs = [Option::<u64>::None; 6];
	let mut xmms = [Option::<f64>::None; 8];

	let mut reg_index = 0;
	let mut xmm_index = 0;

	for arg in args {
		match arg {
			Arg::Int(_) => {
				if reg_index < 6 {
					regs[reg_index] = Some(arg.int());
					reg_index += 1;
				}
			}
			Arg::Double(_) => {
				if xmm_index < 8 {
					xmms[xmm_index] = Some(arg.double());
					xmm_index += 1;
				}
			}
		}
	}

	let int = ffi_invoke_sysv64(
		regs[0].unwrap_or(0), regs[1].unwrap_or(0),
		regs[2].unwrap_or(0), regs[3].unwrap_or(0),
		regs[4].unwrap_or(0), regs[5].unwrap_or(0),
		xmms[0].unwrap_or(0.0), xmms[1].unwrap_or(0.0),
		xmms[2].unwrap_or(0.0), xmms[3].unwrap_or(0.0),
		xmms[4].unwrap_or(0.0), xmms[5].unwrap_or(0.0),
		xmms[6].unwrap_or(0.0), xmms[7].unwrap_or(0.0),
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

#[no_mangle]
pub unsafe extern "C" fn ffi_call_sysv64(
	f: *const c_void,
	len: c_ulonglong, args: *const (c_ulonglong, c_longlong),
	rt: c_int, ret: *mut c_longlong,
) -> c_int {
	assert_eq!(
		core::mem::size_of::<Arg>(),
		core::mem::size_of::<(c_ulonglong, c_longlong)>(),
	);
	for c in 0..len {
		let (c, _) = *args.offset(c as _);
		if c >= 2 {
			return 0;
		}
	}
	let args = core::mem::transmute(args);
	let rv = call(f as _, core::slice::from_raw_parts(args, len as _));
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
