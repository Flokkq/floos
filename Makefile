BINARY_NAME := floos
TARGET_NAME := x86_64-floos

build, b:
	cargo b

test, t:
	cargo t

image:
	cargo bootimage

qemu, q: image
	qemu-system-x86_64 -drive format=raw,file=./target/$(TARGET_NAME)/debug/bootimage-$(BINARY_NAME).bin

ifeq ($(firstword $(MAKECMDGOALS)),usb)
USB_ARG := $(word 2,$(MAKECMDGOALS))

ifeq ($(USB_ARG),)
	$(error you must pass exactly one argument to “make usb”, e.g. “make usb /dev/tty”)
endif

# make the passed-in path a dummy target so Make won’t try to build it
$(eval $(USB_ARG):; @:)
endif

.PHONY: usb
usb, u: image
	dd if=target/$(TARGET_NAME)/debug/bootimage-$(BINARY_NAME).bin of=$(USB_ARG) && sync
