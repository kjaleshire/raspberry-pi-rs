ARMGNU ?= armv7-none-eabi
ARMRUST ?= armv7-unknown-linux-gnueabihf

gcc: blinker01.hex kernel7.img

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

blinker01.o: blinker01.rs
	rustc -O --target $(ARMRUST) -Z no-landing-pads --emit=obj blinker01.rs

blinker01.elf: linker.ld entry.o blinker01.o
	$(ARMGNU)-ld entry.o blinker01.o -T linker.ld -o blinker01.elf
	$(ARMGNU)-objdump -D blinker01.elf > blinker01.list

kernel7.img: blinker01.elf
	$(ARMGNU)-objcopy blinker01.elf -O binary kernel7.img

blinker01.hex : blinker01.elf
	$(ARMGNU)-objcopy blinker01.elf -O ihex blinker01.hex
