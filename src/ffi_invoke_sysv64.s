.globl ffi_invoke_sysv64



ffi_invoke_sysv64:
	sub rsp, 0x80
	push rbp
	mov rbp, rsp

		mov rax, [rsp +0x90] # function
		mov r10, [rsp +0x98] # len
		mov r11, [rsp +0xa0] # args

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

		call rax

	mov rsp, rbp
	pop rbp
	add rsp, 0x80
	ret
