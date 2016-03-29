ARMGNU ?= armv7-none-eabi
ARMRUST ?= armv7-unknown-linux-gnueabihf

gcc: blinker02.hex kernel7.img

all: gcc

clean:
	rm -f *.o
	rm -f *.bin
	rm -f *.hex
	rm -f *.elf
	rm -f *.list
	rm -f *.img
	rm -f *.bc

install: kernel7.img
	cp kernel7.img /Volumes/PIXERMENT/
	diskutil eject /dev/disk2

entry.o : entry.asm
	$(ARMGNU)-as entry.asm -o entry.o

blinker02.o: blinker02.rs
	rustc --target $(ARMRUST) --emit=obj blinker02.rs

blinker02.elf: linker.ld entry.o blinker02.o
	$(ARMGNU)-ld -O0 entry.o blinker02.o -T linker.ld -o blinker02.elf
	$(ARMGNU)-objdump -D blinker02.elf > blinker02.list

kernel7.img: blinker02.elf
	$(ARMGNU)-objcopy blinker02.elf -O binary kernel7.img

blinker02.hex : blinker02.elf
	$(ARMGNU)-objcopy blinker02.elf -O ihex blinker02.hex
