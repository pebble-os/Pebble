export PLATFORM ?= x86_64
export BUILD_DIR ?= $(abspath ./build)
# In the future, this should just be features we can enable, but this is easier in makefile
export KERNEL_FLAGS ?=

IMAGE_NAME ?= pebble.img
QEMU_DIR ?=
QEMU_COMMON_FLAGS = -cpu max,vmware-cpuid-freq,invtsc \
					-machine q35 \
					-smp 2 \
					-m 512M \
					-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
					-device usb-ehci,id=ehci,bus=pcie.0 \
					--no-reboot \
					--no-shutdown \
					-drive if=pflash,format=raw,file=ovmf/OVMF_CODE.fd,readonly \
					-drive if=pflash,format=raw,file=ovmf/OVMF_VARS.fd \
					-drive if=ide,format=raw,file=$(IMAGE_NAME) \
					-serial stdio \
					-usb \
					-net none \
					-display none
# This can be used to pass extra flags to QEMU
QEMU_EXTRA_FLAGS ?=

.PHONY: image_x86_64 prepare kernel test_process simple_fb clean qemu gdb update fmt test echo
.DEFAULT_GOAL := image_$(PLATFORM)

image_x86_64: prepare kernel test_process simple_fb echo
	# Create a temporary image for the FAT partition
	dd if=/dev/zero of=$(BUILD_DIR)/fat.img bs=1M count=64
	mkfs.vfat -F 32 $(BUILD_DIR)/fat.img -n BOOT
	# Copy the stuff into the FAT image
	mcopy -i $(BUILD_DIR)/fat.img -s $(BUILD_DIR)/fat/* ::
	# Create the real image
	dd if=/dev/zero of=$(IMAGE_NAME) bs=512 count=93750
	# Create GPT headers and a single EFI partition
	parted $(IMAGE_NAME) -s -a minimal mklabel gpt
	parted $(IMAGE_NAME) -s -a minimal mkpart EFI FAT32 2048s 93716s
	parted $(IMAGE_NAME) -s -a minimal toggle 1 boot
	# Copy the data from efi.img into the correct place
	dd if=$(BUILD_DIR)/fat.img of=$(IMAGE_NAME) bs=512 count=91669 seek=2048 conv=notrunc
	rm $(BUILD_DIR)/fat.img

prepare:
	@mkdir -p $(BUILD_DIR)/fat/
	cp ovmf/startup.nsh build/fat/startup.nsh

kernel:
	make -C kernel kernel_$(PLATFORM)

test_process:
	cargo xbuild --target=test_process/x86_64-pebble-userspace.json --manifest-path test_process/Cargo.toml
	cp test_process/target/x86_64-pebble-userspace/debug/test_process $(BUILD_DIR)/fat/test_process.elf

simple_fb:
	cargo xbuild --target=drivers/$(PLATFORM)-pebble-userspace.json --manifest-path drivers/simple_fb/Cargo.toml
	cp drivers/target/$(PLATFORM)-pebble-userspace/debug/simple_fb $(BUILD_DIR)/fat/simple_fb.elf

echo:
	cargo xbuild --target=drivers/$(PLATFORM)-pebble-userspace.json --manifest-path drivers/echo/Cargo.toml
	cp drivers/target/$(PLATFORM)-pebble-userspace/debug/echo $(BUILD_DIR)/fat/echo.elf

clean:
	cd drivers && cargo clean
	cd test_process && cargo clean
	make -C kernel clean
	rm -rf build
	rm -f $(IMAGE_NAME)

update:
	cargo update --manifest-path kernel/Cargo.toml
	cargo update --manifest-path drivers/Cargo.toml
	cargo update --manifest-path lib/libpebble/Cargo.toml
	cargo update --manifest-path lib/mer/Cargo.toml
	cargo update --manifest-path lib/pebble_util/Cargo.toml

fmt:
	@# `cargo fmt` doesn't play nicely with conditional compilation, so we manually `rustfmt` things
	find kernel/src -type f -name "*.rs" -exec rustfmt {} +
	cd lib/libpebble && cargo fmt
	cd bootloader && cargo fmt

test:
	cargo test --all-features --manifest-path lib/pebble_util/Cargo.toml
	make -C kernel test

qemu: image_$(PLATFORM)
	$(QEMU_DIR)qemu-system-x86_64 \
		$(QEMU_COMMON_FLAGS) \
		$(QEMU_EXTRA_FLAGS) \
		-enable-kvm

qemu-no-kvm: image_$(PLATFORM)
	$(QEMU_DIR)qemu-system-x86_64 $(QEMU_COMMON_FLAGS) $(QEMU_EXTRA_FLAGS)

debug: image_$(PLATFORM)
	$(QEMU_DIR)qemu-system-x86_64 \
		$(QEMU_COMMON_FLAGS) \
		$(QEMU_EXTRA_FLAGS) \
		-d int

gdb: image_$(PLATFORM)
	$(QEMU_DIR)qemu-system-x86_64 \
		$(QEMU_COMMON_FLAGS) \
		$(QEMU_EXTRA_FLAGS) \
		--enable-kvm \
		-s \
		-S \
	& tools/rust_gdb -q "build/fat/kernel.elf" -ex "target remote :1234"
