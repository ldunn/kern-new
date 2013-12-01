RUST_PATH := .
QEMU := qemu
BOCHS := bochs
SOURCE_SUFFIXES := '(' -name '*.s' -o -name 'main.rs' -o -name 'lib.rs' -o -name 'kmain.rs' ')'
SRCFILES := $(shell find 'src' ${SOURCE_SUFFIXES})
OBJFILES := $(patsubst %.s, %.o, $(patsubst %.rs, %.o, $(SRCFILES)))
$(warning $(OBJFILES))
RUSTFLAGS := --lib --target i386-intel-linux -Z debug-info  --opt-level 0 -A ctypes -L ./src/kernlib -L ./src/userland
LDFLAGS := -nostdlib -g -melf_i386 -z muldefs -static
ASFLAGS := -felf32 -g
.PHONY: all clean qemu rustpkg

all: os.iso

qemu: os.iso
	@$(QEMU) -cdrom os.iso -monitor stdio
bochs: os.iso
	@$(BOCHS)

bochsgui: os.iso
	@$(BOCHS) -f bochsrcgui

os.iso: kernel
	@mkdir -p isofs/System
	cp src/kernel/kernel.bin isofs/System
	genisoimage -R -b boot/grub/stage2_eltorito -no-emul-boot -boot-load-size 4 -input-charset utf-8 -boot-info-table -o $@ isofs


kernel:
	@cd src/kernel/ && $(MAKE)

clean:
	@cd src/kernel && $(MAKE) clean
