.globl ffi_invoke_sysv64



ffi_invoke_sysv64:
	sub rsp, 0x80
	push r12
	push r13
	push r14
	push rbp
	mov rbp, rsp

		# parameter
		mov rax, [rsp +0xa8] # function
		mov r10, [rsp +0xb0] # len
		mov r11, [rsp +0xb8] # args

		# locals
		mov r12, 0x06 # reg_count
		mov r13, 0x08 # xmm_count
		mov r14, r10
		imul r14, 0x08

		test r10, 1
		jnz 4f
			push 0
		4:

		sub rsp, r14
		mov r14, rsp # stack_end

		# body
		2:
		cmp r10, 0 # while len > 0
		jle 0f
			dec r10
			cmp byte ptr [r11 +0x00], 0 # match arg type
			jg 5f
				cmp r12, 0 # if reg_count > 0
				jle 6f
					dec r12
					add r11, 0x10
					jmp 2b
			5:
				cmp r13, 0 # if xmm_count > 0
				jle 6f
					dec r13
					add r11, 0x10
					jmp 2b
			6:
			mov rbx, [r11 +0x08]
			mov [r14], rbx
			add r14, 0x08
			add r11, 0x10
			jmp 2b
		0:

		call rax

	mov rsp, rbp
	pop rbp
	pop r14
	pop r13
	pop r12
	add rsp, 0x80
	ret
