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
