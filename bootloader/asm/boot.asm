section .boot
bits 16
global boot

boot:
	mov ax, 0x2401
	int 0x15 ; enable A20 line
	mov ax, 0x3
	int 0x10 ; set video mode to VGA text mode 3

    mov [disk], dl

    mov ah, 0x2
    mov al, 6
    mov ch, 0
    mov dh, 0
    mov cl, 2
    mov dl, [disk]
    mov bx, copy_target
    int 0x13

	cli

	lgdt [gdt_pointer]
	mov eax, cr0
	or eax,0x1
	mov cr0, eax

    mov ax, DATA_SEG
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	jmp CODE_SEG:boot2

gdt_start:
	dq 0x0
gdt_code:
	dw 0xFFFF
	dw 0x0
	db 0x0
	db 10011010b
	db 11001111b
	db 0x0
gdt_data:
	dw 0xFFFF
	dw 0x0
	db 0x0
	db 10010010b
	db 11001111b
	db 0x0
gdt_end:

gdt_pointer:
	dw gdt_end - gdt_start
	dd gdt_start
disk: db 0x0
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

times 510 - ($-$$) db 0
dw 0xaa55

copy_target:
bits 32

noCPUID:
	hlt

no_long_mode:	
	hlt

check_CPUID:
	pushfd
	pop eax

	mov ecx, eax
	xor eax, 1 << 21

	push eax
	popfd
	pushfd
	pop eax
	
	push ecx
	popfd

	xor eax, ecx
	jz noCPUID
	ret

boot2:
	call check_CPUID
	
	; check if long mode exists
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	jb no_long_mode
	mov eax, 0x80000001
	cpuid
	test edx, 1 << 29
	jz no_long_mode

	; set up paging
	mov edi, 0x1000
	mov cr3, edi
	xor eax, eax
	mov ecx, 4096
	rep stosd
	mov edi, cr3

	mov dword [edi], 0x2003
	add edi, 0x1000
	mov dword [edi], 0x3003
	add edi, 0x1000
	mov dword [edi], 0x4003
	add edi, 0x1000

	mov ebx, 0x00000003 
	mov ecx, 512

	.set_entry:
		mov dword [edi], ebx
		add ebx, 0x1000
		add edi, 8
		loop .set_entry

	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax
	
	mov ecx, 0xC0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	lgdt [GDT.Pointer]
	jmp GDT.Code:Realm64
	hlt

; access bits
PRESENT equ 1 << 7
NOT_SYS equ 1 << 4
EXEC equ 1 << 3
DC equ 1 << 2
RW equ 1 << 1
ACCESSED equ 1 << 0

; flags bits
GRAN_4K equ 1 << 7
SZ_32 equ 1 << 6
LONG_MODE equ 1 << 5

GDT:
	.Null: equ $ - GDT
		dq 0
	.Code: equ $ - GDT
		dd 0xFFFF
		db 0 
		db PRESENT | NOT_SYS | EXEC | RW
		db GRAN_4K | LONG_MODE | 0xF
		db 0
	.Data: equ $ - GDT
		dd 0xFFFF
		db 0
		db PRESENT | NOT_SYS | RW
		db GRAN_4K | SZ_32 | 0xF
		db 0
	.TSS: equ $ - GDT
		dd 0x00000068
        dd 0x00CF8900
	.Pointer:
		dw $ - GDT - 1
		dq GDT

bits 64
hello: db 0

Realm64:
	cli
	mov ax, GDT.Data
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	mov esi,hello
    mov ebx,0xb8000
	
	.loop:
		lodsb
		or al,al
		jz halt
		or eax,0x0F00
		mov word [ebx], ax
		add ebx,2
		jmp .loop

halt:
	mov esp, kernel_stack_top
    extern _start
    call _start
    hlt

section .bss
align 4
kernel_stack_bottom equ $
    resb 16384
kernel_stack_top: