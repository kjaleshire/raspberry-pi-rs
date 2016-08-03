arm_gnu_triple ?= armv7-none-eabi
arm_rust_triple ?= armv7-unknown-linux-gnueabihf
arch ?= armv7

build_dir := build/arch/$(arch)
target_dir := target/$(arm_rust_triple)

kernel_image := $(build_dir)/kernel7.img
rust_object := target/$(arm_rust_triple)/release/libblinker05.a
linker_file := src/arch/$(arch)/linker.ld
assembly_source_files := $(wildcard src/arch/armv7/*.asm)
assembly_object_files := $(patsubst src/arch/armv7/%.asm, $(build_dir)/%.o, $(assembly_source_files))
elf_outfile := $(build_dir)/blinker05.elf
hex_outfile := $(build_dir)/blinker05.hex

all: $(hex_outfile) $(kernel_image)

clean:
	@cargo clean
	@rm -rf build

install: $(kernel_image)
	@cp $(kernel_image) /Volumes/PIXERMENT/
	@diskutil eject /dev/disk2

$(rust_object): src/lib.rs
	@cargo rustc --target $(arm_rust_triple) --release -- -Z no-landing-pads -C no-redzone

$(elf_outfile): $(rust_object) $(assembly_object_files) $(linker_file)
	$(arm_gnu_triple)-ld $(assembly_object_files) $(rust_object) -T $(linker_file) -o $(elf_outfile)
	$(arm_gnu_triple)-objdump -D $(elf_outfile) > $(build_dir)/blinker05.list

$(kernel_image): $(elf_outfile)
	$(arm_gnu_triple)-objcopy $(elf_outfile) -O binary $(kernel_image)

$(hex_outfile): $(elf_outfile)
	$(arm_gnu_triple)-objcopy $(elf_outfile) -O ihex $(hex_outfile)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	$(arm_gnu_triple)-as $< -o $@
