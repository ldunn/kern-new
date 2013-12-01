%macro IRQ 2
	global irq%1
	irq%1:
		cli
		push byte 0
		push byte %2
		jmp irq_common_stub
%endmacro

%macro POPULATE_IRQ 2
    push irq%1
    push %2
    call idt_gate_from_asm
    add esp, 8
%endmacro

IRQ 0, 32
IRQ 1, 33
IRQ 2, 34
IRQ 3, 35
IRQ 4, 36
IRQ 5, 37
IRQ 6, 38
IRQ 7, 39
IRQ 8, 40
IRQ 9, 41
IRQ 10, 42
IRQ 11, 43
IRQ 12, 44
IRQ 13, 45
IRQ 14, 46
IRQ 15, 47

; can't push byte 128, so fuck the macro
irq128:
    cli
    push byte 0
	push 128
	jmp irq_common_stub

extern idt_gate_from_asm
global populate_idt_irq

populate_idt_irq:
    POPULATE_IRQ 0, 32
    POPULATE_IRQ 1, 33
    POPULATE_IRQ 2, 34
    POPULATE_IRQ 3, 35
    POPULATE_IRQ 4, 36
    POPULATE_IRQ 5, 37
    POPULATE_IRQ 6, 38
    POPULATE_IRQ 7, 39
    POPULATE_IRQ 8, 40
    POPULATE_IRQ 9, 41
    POPULATE_IRQ 10, 42
    POPULATE_IRQ 11, 43
    POPULATE_IRQ 12, 44
    POPULATE_IRQ 13, 45
    POPULATE_IRQ 14, 46
    POPULATE_IRQ 15, 47
    push irq128
    push 128
    call idt_gate_from_asm
    add esp, 8
    ret

[EXTERN irq_handler]

irq_common_stub:

	pusha

	mov ax, ds
	push eax

	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
    call irq_handler
	pop ebx
    pop ecx
	mov ds, bx
	mov es, bx
	mov fs, bx
	mov gs, bx
    add esp, 32 ;; A bit of stack fuckery to allow us to return eax
    push eax
    sub esp, 32
    popa
    pop eax
    add esp, 4
    sti
	iret
