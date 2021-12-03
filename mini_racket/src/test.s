	global _entry
	default rel
	section .text
	global _entry
_entry:
	mov rax, 3
	cmp rax, 3
	je _d9061cc2ab1c486d82f92c3a9844e407
	mov rax, 1
	jmp _d015f0dad3c44ed5a1b32ef4417e9a03
_d9061cc2ab1c486d82f92c3a9844e407:
	mov rax, 4
	push rax
	mov rax, 10
	push rax
	mov rax, 16
	pop r8
	sub r8, rax
	mov rax, r8
	sub rax, 2
	pop r8
	add rax, r8
_d015f0dad3c44ed5a1b32ef4417e9a03:
	ret