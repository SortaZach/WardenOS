BOOTLOADER = bootloader/target/x86_64-unknown-uefi/debug/bootloader.efi
TARGET_DIR = boot/EFI/BOOT
TARGET = $(TARGET_DIR)/BOOT64.efi
QEMU = qemu-system-x86_64
BIOS = /usr/share/qemu/OVMF.fd
DRIVE = fat:rw:$(HOME)/WardenOS/boot

all: run

$(TARGET): $(BOOTLOADER)
	@mkdir -p $(TARGET_DIR)
	cp $(BOOTLOADER) $(TARGET)

run: $(TARGET)
	$(QEMU) -bios $(BIOS) -drive format=raw,file=$(DRIVE)

clean: 
	rm -f $(TARGET)

.PHONY: all run clean
