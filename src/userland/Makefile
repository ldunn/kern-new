RUST := rustc
RUST_PATH := ../..
AS := nasm
LD := ld
SOURCE_SUFFIXES := '(' -name '*.s' -o -name 'main.rs' ')'
SRCFILES := $(shell find '.' ${SOURCE_SUFFIXES})
OBJFILES := $(patsubst %.s, %.o, $(patsubst %.rs, %.o, $(SRCFILES)))
$(warning $(OBJFILES))
RUSTFLAGS := --target i386-intel-linux -Z debug-info  --opt-level 0 -A ctypes --lib
LDFLAGS := -nostdlib -g -melf_i386 -z muldefs
ASFLAGS := -felf32 -g
LIBRARY := ../kernlib/kernlib.a
.PHONY: all clean qemu rustpkg

all: main.elf

main.elf: ${OBJFILES}
	$(LD) $(LDFLAGS) -T linker.ld -o $@ ${OBJFILES}

%.o: %.rs
	$(RUST) ${RUSTFLAGS} -c $< -o $@


clean:
	$(RM) $(wildcard $(OBJFILES) main.elf)
