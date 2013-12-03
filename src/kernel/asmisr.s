;; From jamesmolloy
%macro ISR_NOERRCODE 1
	[GLOBAL isr%1]
	isr%1:
		cli	
		push byte 0
		push byte %1
		jmp isr_common_stub
%endmacro

%macro ISR_ERRCODE 1
	[GLOBAL isr%1]
	isr%1:
		cli
		push byte %1
		jmp isr_common_stub
%endmacro

%macro POPULATE_ISR 1
    push isr%1
    push %1
    call idt_gate_from_asm
    add esp, 8
%endmacro

ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 2
ISR_NOERRCODE 3
ISR_NOERRCODE 4
ISR_NOERRCODE 5
ISR_NOERRCODE 6
ISR_NOERRCODE 7
ISR_ERRCODE 8
ISR_NOERRCODE 9
ISR_ERRCODE 10
ISR_ERRCODE 11
ISR_ERRCODE 12
ISR_ERRCODE 13
ISR_ERRCODE 14
ISR_NOERRCODE 15
ISR_NOERRCODE 16
ISR_NOERRCODE 17
ISR_NOERRCODE 18
ISR_NOERRCODE 19
ISR_NOERRCODE 20
ISR_NOERRCODE 21
ISR_NOERRCODE 22
ISR_NOERRCODE 23
ISR_NOERRCODE 24
ISR_NOERRCODE 25
ISR_NOERRCODE 26
ISR_NOERRCODE 27
ISR_NOERRCODE 28
ISR_NOERRCODE 29
ISR_NOERRCODE 30
ISR_NOERRCODE 31

extern idt_gate_from_asm
global populate_idt_isr

populate_idt_isr:
    POPULATE_ISR 0
    POPULATE_ISR 1
    POPULATE_ISR 2
    POPULATE_ISR 3
    POPULATE_ISR 4
    POPULATE_ISR 5
    POPULATE_ISR 6
    POPULATE_ISR 7
    POPULATE_ISR 8
    POPULATE_ISR 9
    POPULATE_ISR 10
    POPULATE_ISR 11
    POPULATE_ISR 12
    POPULATE_ISR 13
    POPULATE_ISR 14
    POPULATE_ISR 15
    POPULATE_ISR 16
    POPULATE_ISR 17
    POPULATE_ISR 18
    POPULATE_ISR 19
    POPULATE_ISR 20
    POPULATE_ISR 21
    POPULATE_ISR 22
    POPULATE_ISR 23
    POPULATE_ISR 24
    POPULATE_ISR 25
    POPULATE_ISR 26
    POPULATE_ISR 27
    POPULATE_ISR 28
    POPULATE_ISR 29
    POPULATE_ISR 30
    POPULATE_ISR 31
    ret
extern isr_handler

isr_common_stub:
   pusha                    ; Pushes edi,esi,ebp,esp,ebx,edx,ecx,eax

   mov ax, ds               ; Lower 16-bits of eax = ds.
   push eax                 ; save the data segment descriptor

   mov ax, 0x10  ; 1oad the kernel data segment descriptor
   mov ds, ax
   mov es, ax
   mov fs, ax
   mov gs, ax
    call isr_handler

   pop eax        ; re1oad the original data segment descriptor
   mov ds, ax
   mov es, ax
   mov fs, ax
   mov gs, ax
    mov [gs:0x30], dword 0
   add esp, 8     ; C1eans up the pushed error code and pushed ISR number
   popa                     ; Pops edi,esi,ebp...
   sti
   iret           ; pops 5 things at once: CS, EIP, EFLAGS, SS, and ESP
