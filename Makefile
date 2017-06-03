ARCH		= x86_64
TARGET		= $(ARCH)-unknown-whetstone
BUILD_ROOT	= build
KERNEL		= bootx64.efi
HD_IMG		= boot.img

LOADER		= loader/target/$(TARGET)/debug/libuefi_loader.a
KERNEL_OBJ	= target/$(TARGET)/debug/libwhetstone.a

FORMAT		= efi-app-$(ARCH)
LDFLAGS		= --gc-sections --oformat pei-x86-64 --subsystem 10 -pie -e efi_main

prefix		= x86_64-efi-pe-
CC			= gcc
CXX			= g++
#CC			= $(prefix)gcc
#CXX			= $(prefix)g++
LD			= $(prefix)ld
AS			= $(prefix)as
AR			= $(prefix)ar
OBJCOPY		= $(prefix)objcopy
MFORMAT		= mformat
MMD			= mmd
MCOPY		= mcopy
RUSTC		= rustc
CARGO		= xargo

LOADER_SRC	= $(wildcard loader/*.rs)
LIBUEFI_SRC = $(wildcard external/uefi/src/*.rs)

BUILD_TARGET = $(BUILD_ROOT)/$(KERNEL)

.PHONY: all clean iso cargo

all: $(BUILD_TARGET)

$(LOADER): $(LOADER_SRC) $(LIBUEFI_SRC)
	$(CARGO) build --manifest-path=loader/Cargo.toml --target $(TARGET)
	cd loader/target/$(TARGET)/debug && $(AR) x libuefi_loader.a

$(BUILD_TARGET): $(LOADER)
	@mkdir -p $(BUILD_ROOT)
	$(LD) $(LDFLAGS) -o $@ $(dir $(LOADER))*.o

$(KERNEL_OBJ): $(LOADER)
#	RUSTFLAGS='-L $(LIBUEFI_DIR) -L $(LIBCORE_DIR)' $(CARGO) build --target $(TARGET)

img: $(BUILD_ROOT)/$(HD_IMG)

$(BUILD_ROOT)/$(HD_IMG): $(BUILD_ROOT)/$(KERNEL)
	@dd if=/dev/zero of=fat.img bs=1k count=1440
	@$(MFORMAT) -i fat.img -f 1440 ::
	@$(MMD) -i fat.img ::/EFI
	@$(MMD) -i fat.img ::/EFI/BOOT
	@$(MCOPY) -i fat.img $(BUILD_ROOT)/$(KERNEL) ::/EFI/BOOT
	@mv fat.img $(BUILD_ROOT)/$(HD_IMG)

run: img
	qemu-system-x86_64 -enable-kvm -net none -m 1024 -bios ovmf.fd -usb -usbdevice disk::$(BUILD_ROOT)/$(HD_IMG)

clean:
	@cd loader && $(CARGO) clean && rm -rf target
	@$(CARGO) clean
	@rm -rf build fat.img
