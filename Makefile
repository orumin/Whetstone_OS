ARCH		= x86_64
TARGET		= $(ARCH)-unknown-whetstone
BUILD_ROOT	= build
KERNEL		= bootx64.efi
ISO			= boot.iso

LIBCORE_DIR	= external/core/target/$(TARGET)/debug/
LIBCORE		= $(LIBCORE_DIR)/libcore.rlib
LIBUEFI_DIR	= external/uefi/target/$(TARGET)/debug/
LIBUEFI		= $(LIBUEFI_DIR)/libuefi.rlib
LOADER		= loader/target/$(TARGET)/debug/libuefi_loader.a
KERNEL_OBJ	= target/$(TARGET)/debug/libwhetstone.a

FORMAT		= efi-app-$(ARCH)
LDFLAGS		= --oformat pei-x86-64 --subsystem 10 -pie -e efi_main

prefix		= x86_64-efi-pe-
CC			= gcc
CXX			= g++
#CC			= $(prefix)gcc
#CXX			= $(prefix)g++
LD			= $(prefix)ld
AS			= $(prefix)as
AR			= $(prefix)ar
OBJCOPY		= $(prefix)objcopy
RUSTC		= rustc
CARGO		= xargo

LOADER_SRC	= $(wildcard loader/*.rs)

BUILD_TARGET = $(BUILD_ROOT)/$(KERNEL)

.PHONY: all clean iso cargo

all: $(BUILD_TARGET)

$(LOADER): $(LOADER_SRC)
	$(CARGO) build --manifest-path=loader/Cargo.toml --target $(TARGET)
	cd loader/target/$(TARGET)/debug && $(AR) x libuefi_loader.a

$(BUILD_TARGET): $(LOADER)
	@mkdir -p $(BUILD_ROOT)
	$(LD) $(LDFLAGS) -o $@ $(dir $(LOADER))*.o

$(KERNEL_OBJ): $(LOADER)
#	RUSTFLAGS='-L $(LIBUEFI_DIR) -L $(LIBCORE_DIR)' $(CARGO) build --target $(TARGET)

iso: $(BUILD_ROOT)/$(ISO)

$(BUILD_ROOT)/$(ISO): $(BUILD_ROOT)/$(KERNEL)
	@mkdir -p $(BUILD_ROOT)/img/EFI/Boot
	@cp $(BUILD_ROOT)/$(KERNEL) $(BUILD_ROOT)/img/EFI/Boot/
	@cd $(BUILD_ROOT) && mkisofs -o $(ISO) img

run: iso
	qemu-system-x86_64 -enable-kvm -net none -m 1024 -bios ovmf.fd -cdrom ./build/boot.iso -hda fat:./build

clean:
	@cd loader && $(CARGO) clean && rm -rf target
#	@cd external/core && $(CARGO) clean && rm -rf target
#	@cd external/uefi && $(CARGO) clean && rm -rf target
	@$(CARGO) clean
	@rm -rf build
