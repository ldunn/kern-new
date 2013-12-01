global setIdt
setIdt:
 push eax
 mov eax, [esp + 0x8]
 lidt [eax]
 pop eax
 ret
