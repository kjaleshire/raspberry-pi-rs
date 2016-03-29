arm_gnu_triple ?= armv7-none-eabi
arm_rust_triple ?= armv7-unknown-linux-gnueabihf
arch ?= armv7

build_dir := build/arch/$(arch)
target_dir := target/$(arm_rust_triple)

kernel_image := $(build_dir)/kernel7.img
rust_object := target/$(arm_rust_triple)/debug/libblinker03.a
assembly_source_files := $(wildcard src/arch/armv7/*.asm)
assembly_object_files := $(patsubst src/arch/armv7/%.asm, $(build_dir)/%.o, $(assembly_source_files))
elf_outfile := $(build_dir)/blinker03.elf
hex_outfile := $(build_dir)/blinker03.hex

all: $(hex_outfile) $(kernel_image)

clean:
	@cargo clean
	@rm -rf build

install: $(kernel_image)
	cp $(kernel_image) /Volumes/PIXERMENT/
	diskutil eject /dev/disk2

$(rust_object): src/lib.rs
	@cargo rustc --target $(arm_rust_triple) -- -Z no-landing-pads -C no-redzone

$(elf_outfile): $(rust_object) $(assembly_object_files) src/arch/$(arch)/linker.ld
	$(arm_gnu_triple)-ld -O0 $(assembly_object_files) $(rust_object) -T src/arch/$(arch)/linker.ld -o $(elf_outfile)
	$(arm_gnu_triple)-objdump -D $(elf_outfile) > $(build_dir)/blinker03.list

$(kernel_image): $(elf_outfile)
	$(arm_gnu_triple)-objcopy $(elf_outfile) -O binary $(kernel_image)

$(hex_outfile): $(elf_outfile)
	$(arm_gnu_triple)-objcopy $(elf_outfile) -O ihex $(hex_outfile)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	$(arm_gnu_triple)-as $< -o $@
