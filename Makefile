ARCH ?= aarch64
MACHINE ?= shyper
PROFILE ?= release

FS ?= fat

# Cargo flags.
ifeq (${PROFILE}, release)
CARGO_FLAGS = --release
else
CARGO_FLAGS = 
endif

# Target directory.
KERNEL_STD := target/${ARCH}-unknown-shyper/${PROFILE}/rustfstest

TARGET_CFG := ../unishyper/cfg/${ARCH}${MACHINE}.json
KERNEL_ALLOC := target/${ARCH}${MACHINE}/${PROFILE}/rustfstest

# Arch-specific tools
OBJCOPY := rust-objcopy
OBJDUMP := rust-objdump

.PHONY: build clean emu

linux:
	cargo build ${CARGO_FLAGS}

build_unishyper_std:
	cargo +stage1 build -Zbuild-std=std,panic_unwind -Zbuild-std-features=compiler-builtins-mem --target ${ARCH}-unknown-shyper ${CARGO_FLAGS} --no-default-features --features "unishyper-std,${FS},${MACHINE}"
	${OBJCOPY} ${KERNEL_STD} -O binary ${KERNEL_STD}_unishyperstd.bin
	${OBJDUMP} --demangle -d ${KERNEL_STD} > ${KERNEL_STD}_unishyperstd.asm
	cp ${KERNEL_STD}_unishyperstd.bin unishyperstd${FS}.bin

build_unishyper_alloc:
	cargo build --target ${TARGET_CFG} -Z build-std=core,alloc -Zbuild-std-features=compiler-builtins-mem ${CARGO_FLAGS} --no-default-features --features "unishyper-alloc,${FS},${MACHINE}"
	${OBJCOPY} ${KERNEL_ALLOC} -O binary ${KERNEL_ALLOC}_unishyperalloc.bin
	${OBJDUMP} --demangle -d ${KERNEL_ALLOC} > ${KERNEL_ALLOC}_unishyperalloc.asm
	cp ${KERNEL_ALLOC}_unishyperalloc.bin ./unishyperalloc${FS}.bin

QEMU_DISK_OPTIONS := -drive file=disk.img,if=none,format=raw,id=x0 \
					 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
					 -global virtio-mmio.force-legacy=false

QEMU_COMMON_OPTIONS := -serial stdio -display none -smp 4 -m 2048

unishyper_std: build_unishyper_std
	sudo qemu-system-aarch64 -M virt -cpu cortex-a53 \
		${QEMU_COMMON_OPTIONS} \
		${QEMU_DISK_OPTIONS} \
		-kernel ${KERNEL_STD}_unishyperstd.bin -s

unishyper_alloc: build_unishyper_alloc
	sudo qemu-system-aarch64 -M virt -cpu cortex-a53 \
		${QEMU_COMMON_OPTIONS} \
		${QEMU_DISK_OPTIONS} \
		-kernel ${KERNEL_ALLOC}_unishyperalloc.bin -s

debug: build
	sudo qemu-system-aarch64 -M virt -cpu cortex-a53 \
		${QEMU_COMMON_OPTIONS} \
		${QEMU_DISK_OPTIONS} \
		-kernel ${KERNEL_STD}.bin -s -S

clean:
	-cargo clean