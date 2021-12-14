	global _entry
	default rel
	section .text
	global _entry
_entry:
	mov rax, 4
	push rax
	mov rax, 2
	push rax
	mov rax, 10
	pop r8
	sub r8, rax
	mov rax, r8
	push rax
	mov rax, [rsp + 8]
	push rax
	mov rax, 3
	cmp rax, 3
	je _g0
	mov rax, 2
	jmp _g1
_g0:
	mov rax, [rsp + 8]
_g1:
	pop r8
	add rax, r8
	add rsp, 8
	add rsp, 8
	ret