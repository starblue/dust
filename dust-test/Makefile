BINARIES =
BINARIES += test-port
BINARIES += test-systick
BINARIES += test-uart
BINARIES += test-plu
BINARIES += test-all-pins
BINARIES += test-hardfault-handler

DEVICES =
DEVICES += atsamd09c13 atsamd09d14
DEVICES += atsaml11e15
DEVICES += lpc802m001 lpc802m011 lpc804m101 lpc804m111  lpc810 lpc812 lpc822 lpc824 lpc832 lpc845
DEVICES += lpc1114
DEVICES += lpc1343
DEVICES += stm32f030x6 stm32f070x6

test-port_DEVICES = $(DEVICES)
test-systick_DEVICES = $(DEVICES)
test-uart_DEVICES = lpc802m001 lpc802m011 lpc804m101 lpc804m111 lpc810 lpc812 lpc822 lpc824 lpc832 lpc845
test-plu_DEVICES = lpc804m101
test-all-pins_DEVICES = lpc802m001 lpc804m101 lpc822 lpc824 lpc845
test-hardfault-handler_DEVICES = lpc845

LD_DIR = ldscripts
OUTPUT_DIR = output


CARGOFLAGS =
CARGOFLAGS += --color never
CARGOFLAGS += --release
#CARGOFLAGS += --verbose

EXECUTABLES = $(foreach B,$(BINARIES),$(foreach D,$($(B)_DEVICES),$D_$B))

BUILDS = $(addprefix build_, $(EXECUTABLES))
HEX_FILES = $(addprefix $(OUTPUT_DIR)/, $(addsuffix .hex, $(EXECUTABLES)))

atsamd09c13_TARGET = thumbv6m-none-eabi
atsamd09d14_TARGET = thumbv6m-none-eabi
atsaml11e15_TARGET = thumbv6m-none-eabi
lpc802m001_TARGET = thumbv6m-none-eabi
lpc802m011_TARGET = thumbv6m-none-eabi
lpc804m101_TARGET = thumbv6m-none-eabi
lpc804m111_TARGET = thumbv6m-none-eabi
lpc810_TARGET = thumbv6m-none-eabi
lpc812_TARGET = thumbv6m-none-eabi
lpc822_TARGET = thumbv6m-none-eabi
lpc824_TARGET = thumbv6m-none-eabi
lpc832_TARGET = thumbv6m-none-eabi
lpc845_TARGET = thumbv6m-none-eabi
lpc1114_TARGET = thumbv6m-none-eabi
lpc1343_TARGET = thumbv7m-none-eabi
stm32f030x6_TARGET = thumbv6m-none-eabi
stm32f070x6_TARGET = thumbv6m-none-eabi

RUSTFLAGS =
RUSTFLAGS += -C link-arg=-T$(LD_DIR)/$(2).ld
RUSTFLAGS += -C link-arg=-L$(LD_DIR)
RUSTFLAGS += -C link-arg=-Map=$(OUTPUT_DIR)/$(2)_$(1).map
RUSTFLAGS += -C opt-level=z
#RUSTFLAGS += --emit asm,llvm-bc,llvm-ir,obj,metadata,link,dep-info,mir


all: $(HEX_FILES)

define b_d_rules =
$(1):: $(OUTPUT_DIR)/$(2)_$(1).hex
$(2):: $(OUTPUT_DIR)/$(2)_$(1).hex

build_$(2)_$(1):
	RUSTFLAGS="$(RUSTFLAGS)" \
		cargo build $(CARGOFLAGS) --target $($(2)_TARGET) --features "$(2)" --bin $(1)

$(OUTPUT_DIR)/$(2)_$(1).hex: build_$(2)_$(1)
	arm-none-eabi-objcopy -O ihex target/$($(2)_TARGET)/release/$(1) "$(OUTPUT_DIR)/$(2)_$(1)".hex
endef
$(foreach B,$(BINARIES),$(foreach D,$($(B)_DEVICES),$(eval $(call b_d_rules,$B,$D))))

flash_%: $(OUTPUT_DIR)/%.hex
	lpc21isp -control "$(OUTPUT_DIR)/$*".hex /dev/ttyUSB0 115200 12000

clean:
	cargo clean
	find . \( -name \*~ -o -name \*.rs.bk \) -exec rm {} \;

realclean: clean
	rm -f $(HEX_FILES)
