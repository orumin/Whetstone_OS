ARCH		= x86_64
TARGET		= $(ARCH)-unknown-whetstone
UEFI_TARGET = $(ARCH)-uefi
BUILD_ROOT	= build
ESP_DIR		= $(BUILD_ROOT)/esp
DEST_DIR    = $(ESP_DIR)/efi/boot
TARGET_DIR  = $(BUILD_ROOT)/run
KERNEL		= bootx64.efi

LOADER		= loader/target/$(UEFI_TARGET)/debug/uefi_loader.efi
KERNEL_OBJ	= target/$(TARGET)/debug/libwhetstone.a

RUSTC		= rustc
CARGO		= cargo

OVMF_CODE   = OVMF_CODE.fd
OVMF_VARS   = OVMF_VARS.fd

QEMU		= qemu-system-x86_64
BOOT_OPTS	= -nodefaults -vga std -machine q35,accel=kvm:tcg -m 1024 \
				-drive if=pflash,format=raw,file=$(OVMF_CODE),readonly=on \
				-drive if=pflash,format=raw,file=$(OVMF_VARS),readonly=on \
				-drive format=raw,file=fat:rw:$(ESP_DIR) \
				-drive format=raw,file=fat:rw:$(TARGET_DIR)

# for OVMF debug
# -debugcon file:debug.log -global isa-debugcon.iobase=0x402

LOADER_SRC	= $(wildcard loader/src/*.rs)

BUILD_TARGET = $(DEST_DIR)/$(KERNEL)

.PHONY: all clean iso cargo

all: $(BUILD_TARGET)

$(LOADER): $(LOADER_SRC)
	cd loader && \
	RUST_TARGET_PATH=`pwd` $(CARGO) xbuild --target $(UEFI_TARGET)

$(BUILD_TARGET): $(LOADER)
	@mkdir -p $(DEST_DIR)
	@mkdir -p $(TARGET_DIR)
	cp $< $@

$(KERNEL_OBJ): $(LOADER)
#	RUSTFLAGS='-L $(LIBUEFI_DIR) -L $(LIBCORE_DIR)' $(CARGO) build --target $(TARGET)

run: $(BUILD_TARGET)
	$(QEMU) $(BOOT_OPTS)

clean:
	@cd loader && $(CARGO) clean && rm -rf target
	@$(CARGO) clean
	@rm -rf build
