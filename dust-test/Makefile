BINARIES =
BINARIES += test-port
BINARIES += test-plu

DEVICES =
DEVICES += lpc802 lpc804 lpc810 lpc812 lpc824 lpc845
DEVICES += lpc1114
DEVICES += lpc1343

test-port_DEVICES = $(DEVICES)
test-plu_DEVICES = lpc804

CARGOFLAGS =
CARGOFLAGS += --color never
CARGOFLAGS += --release
#CARGOFLAGS += --verbose

EXECUTABLES = $(foreach B,$(BINARIES),$(foreach D,$($(B)_DEVICES),$B_$D))

BUILDS = $(addprefix build_, $(EXECUTABLES))
HEX_FILES = $(addsuffix .hex, $(EXECUTABLES))

lpc802_TARGET = thumbv6m-none-eabi
lpc804_TARGET = thumbv6m-none-eabi
lpc810_TARGET = thumbv6m-none-eabi
lpc812_TARGET = thumbv6m-none-eabi
lpc824_TARGET = thumbv6m-none-eabi
lpc845_TARGET = thumbv6m-none-eabi
lpc1114_TARGET = thumbv6m-none-eabi
lpc1343_TARGET = thumbv7m-none-eabi


RUSTFLAGS =
#RUSTFLAGS += -C linker=lld
#RUSTFLAGS += -Z linker-flavor=ld.lld
RUSTFLAGS += -C link-arg=-nostartfiles
#RUSTFLAGS += --emit asm,llvm-bc,llvm-ir,obj,metadata,link,dep-info,mir


all: $(HEX_FILES)

clean:
	cargo clean
	rm -f $(HEX_FILES)
	find . \( -name \*~ -o -name \*.rs.bk \) -exec rm {} \;

define b_d_rules =
$(1):: $(1)_$(2).hex
$(2):: $(1)_$(2).hex

build_$(1)_$(2):
	RUSTFLAGS="-C link-arg=-Tlayout_$(2).ld $(RUSTFLAGS)" \
		cargo build $(CARGOFLAGS) --target $($(2)_TARGET) --features "$(2)" --bin $(1)

$(1)_$(2).hex: build_$(1)_$(2)
	arm-none-eabi-objcopy -O ihex target/$($(2)_TARGET)/release/$(1) "$(2)_$(1)".hex
endef
$(foreach B,$(BINARIES),$(foreach D,$($(B)_DEVICES),$(eval $(call b_d_rules,$B,$D))))

flash_%: %.hex
	lpc21isp -control "$*".hex /dev/ttyUSB0 115200 12000
