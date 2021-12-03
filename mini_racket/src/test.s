	global _entry
	default rel
	section .text
	global _entry
_entry:
	mov rax, 1
	cmp rax, 3
	je _146332fb28f844ad8dd428b2189b446a
	mov rax, 1
	jmp _883284a57d5340fbb8dd89ddd917b802
_146332fb28f844ad8dd428b2189b446a:
	mov rax, 3
_883284a57d5340fbb8dd89ddd917b802:
	cmp rax, 3
	je _ce6ceaa049364b7d89342cef5214978d
	mov rax, 84
	jmp _4a22a2bab2574b709ad61abac75171ae
_ce6ceaa049364b7d89342cef5214978d:
	mov rax, 88
_4a22a2bab2574b709ad61abac75171ae:
	ret