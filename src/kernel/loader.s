global loader
global magic
global mbd

global set_gdt
global set_idt
global populate_idt
global reloadSegments
global hang
global outportb
global inportb
global enable_paging
global stack_end
global tss_flush
global jump_usermode

extern kmain
extern test_usermode

; Multiboot header
MODULEALIGN equ 1 << 0
MEMINFO		equ 1 << 1
FLAGS		equ MODULEALIGN | MEMINFO
MAGIC		equ 0x1BADB002
CHECKSUM	equ -(MAGIC+FLAGS)

section .mboot

align 4
	dd MAGIC
	  dd FLAGS
	  dd CHECKSUM

gdtr dw 0
     dd 0

idtr dw 0
     dd 0

STACKSIZE equ 0x4000 ; 16k 

extern _end

loader:
	mov esp, stack_end
	mov [magic], eax
	mov [mbd], ebx
    ; rust functions compare esp against [gs:0x30] as a sort of stack guard thing
    ; as long as we set [gs:0x30] to dword 0, it should be ok
    mov [gs:0x30], dword 0
    ; clear the screen a slightly different colour
    rep stosb
    ; jump into rust
    push eax
    push ebx
    call kmain
    jmp $

	cli

	dd 0

outportb:
    ; Because I can't get Rust's inline assembly to work, all arguments are at a single pointer, at ebp+8
    push ebp
    mov dx, [esp+12]
    mov al, [esp+8]
    out dx, al
    pop ebp
    ret

inportb:
    push ebp
    mov dx, [esp+8]
    in al, dx
    pop ebp
    ret


set_gdt:
    push ebp
    mov eax, [esp+8]
    mov [gdtr+2], eax
    mov cx, [esp+12]
    mov [gdtr], cx
    lgdt [gdtr]
    jmp 0x08:.reload_CS
.reload_CS:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    pop ebp
    ret

set_idt:
    push ebp
    mov eax, [esp+8]
    mov [idtr+2], eax
    mov cx, [eax+12]
    mov [idtr], cx
    mov edx, idtr
    lidt [idtr]
    pop ebp
    sti
    ret

enable_paging:
    push ebp
    mov eax, [esp+8]
    mov cr3, eax
    mov eax, cr0
    or eax, 0x80000000
    mov cr0, eax
    pop ebp
    ret

tss_flush:
    push ebp
    mov ax, 0x2b
    ltr ax
    pop ebp
    ret

extern test_usermode

jump_usermode:
    cli
    push ebp
    mov ebx, [esp+8]
    pop ebp
    mov ax, 0x23
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov eax, esp
    push 0x23
    push eax
    pushf
    pop eax
    or eax, 0x200
    push eax
    push 0x1b
    push ebx
    iret

hang:
	hlt
	jmp hang

section .bss

align 4
stack_begin:
    resb STACKSIZE
stack_end:
magic: resd 1
mbd: resd 1
