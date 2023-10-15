.globl ffi_invoke_win64



ffi_invoke_win64:
	push rbp
	mov rbp, rsp

		mov rax, [rsp +0x30]
		mov r10, [rsp +0x38]
		mov r11, [rsp +0x40]

		test r10, 1
		jz 4f
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

	mov rsp, rbp
	pop rbp
	ret
