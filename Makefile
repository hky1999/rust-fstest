ARCH ?= aarch64
MACHINE ?= shyper
PROFILE ?= release

# Cargo flags.
ifeq (${PROFILE}, release)
CARGO_FLAGS = --release
else
CARGO_FLAGS = 
endif

# Target directory.
KERNEL := target/${ARCH}-unknown-shyper/${PROFILE}/rustfstest

# Arch-specific tools
OBJCOPY := rust-objcopy
OBJDUMP := rust-objdump

.PHONY: build clean emu

linux:
	cargo build ${CARGO_FLAGS}

build_unishyper_std:
	cargo +stage1 build -Zbuild-std=std,panic_unwind -Zbuild-std-features=compiler-builtins-mem --target ${ARCH}-unknown-shyper ${CARGO_FLAGS} --features "std, unishyper-std"
	${OBJCOPY} ${KERNEL} -O binary ${KERNEL}.bin
	${OBJDUMP} --demangle -d ${KERNEL} > ${KERNEL}.asm

build_unishyper_alloc:
	cargo build ${CARGO_FLAGS}

QEMU_DISK_OPTIONS := -drive file=disk.img,if=none,format=raw,id=x0 \
					 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
					 -global virtio-mmio.force-legacy=false

QEMU_COMMON_OPTIONS := -serial stdio -display none -smp 4 -m 2048

unishyper_std: build_unishyper_std
	sudo qemu-system-aarch64 -M virt -cpu cortex-a53 \
		${QEMU_COMMON_OPTIONS} \
		${QEMU_DISK_OPTIONS} \
		-kernel ${KERNEL}.bin -s

debug: build
	sudo qemu-system-aarch64 -M virt -cpu cortex-a53 \
		${QEMU_COMMON_OPTIONS} \
		${QEMU_DISK_OPTIONS} \
		-kernel ${KERNEL}.bin -s -S

clean:
	-cargo clean