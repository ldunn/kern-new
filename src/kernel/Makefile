RUSTC= rustc
AS := nasm
LD := ld
QEMU := qemu
BOCHS := bochs
SOURCE_SUFFIXES := '(' -name '*.s' -o -name 'kmain.rs' ')'
SRCFILES := $(shell find '.' ${SOURCE_SUFFIXES})
OBJFILES := $(patsubst %.s, %.o, $(patsubst %.rs, %.o, $(SRCFILES)))
$(warning $(OBJFILES))
RUSTFLAGS := --lib --target i386-intel-linux -Z debug-info  --opt-level 2 -A ctypes
LDFLAGS := -nostdlib -g -melf_i386 -z muldefs
ASFLAGS := -felf32 -g
.PHONY: all clean

all: kernel.bin

kernel.bin: ${OBJFILES}
	${LD} ${LDFLAGS} -T linker.ld -o $@ ${OBJFILES} $(shell $(GCC_PREFIX)gcc -print-file-name=32/libgcc.a)

%.o: %.rs
	@$(RUSTC) ${RUSTFLAGS} -c $< -o $@

clean:
	$(RM) $(wildcard $(OBJFILES) kernel.bin)
