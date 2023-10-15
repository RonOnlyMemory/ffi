.globl ffi_invoke_win64



ffi_invoke_win64:
	push rbp
	push r12
		mov rbp, rsp

		mov rax, [rsp +0x38]
		mov r12, [rsp +0x40]
		mov r10, r12
		mov r11, [rsp +0x48]

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

		sub rsp, 0x20
			call rax
		add rsp, 0x20

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
	ret
