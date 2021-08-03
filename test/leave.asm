BITS 32
	org 0x7c00
start:
	call func
	jmp 0
my_add:
	push ebp
	mov ebp,esp
	mov edx,[ebp+0x8]
	mov eax,[ebp+0xc]
	add eax,edx
	pop ebp
	ret
func:
	push ebp
	mov ebp,esp
	sub esp, byte +0x10
	push byte +0x5
	push byte +0x3
	call my_add
	add esp, byte +0x8
	mov [ebp-0x4],eax
	leave
	ret
