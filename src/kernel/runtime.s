use32

global __morestack
global _GLOBAL_OFFSET_TABLE
global abort
global memcmp
global memcpy
global start
global upcall_call_shim_on_c_stack
global upcall_call_shim_on_rust_stack
global upcall_reset_stack_limit
global _Unwind_Resume
global memset
global upcall_rust_personality

upcall_rust_personality:
    leave
    ret

_Unwind_Resume:
    leave
    ret

_GLOBAL_OFFSET_TABLE_:
__morestack:

abort:
    mov eax, 0xdeada
    jmp $

memcmp:
    mov eax, 0xDEADb
    jmp $

upcall_reset_stack_limit:
    leave
    ret

upcall_call_shim_on_c_stack:
    ;; This function takes a function pointer and a pointer to whatever args are passed.
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    mov edx, [ebp+12]
    push eax
    call edx
    mov esp, ebp
    pop ebp
    ret

upcall_call_shim_on_rust_stack:
    ;; This function takes a function pointer and a pointer to whatever args are passed.
    push ebp
    mov ebp, esp
    mov eax, [ebp+8]
    mov edx, [ebp+12]
    push eax
    call edx
    mov esp, ebp
    pop ebp
    ret
