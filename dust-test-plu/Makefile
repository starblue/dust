BINARIES =
BINARIES += dust-test-plu

DEVICES =
DEVICES += lpc804

CARGOFLAGS =
CARGOFLAGS += --color never
CARGOFLAGS += --release
#CARGOFLAGS += --verbose

EXECUTABLES = $(foreach D,$(DEVICES),$(foreach B,$(BINARIES),$D_$B))

BUILDS = $(addprefix build_, $(EXECUTABLES))
HEX_FILES = $(addsuffix .hex, $(EXECUTABLES))

lpc804_TARGET = thumbv6m-none-eabi


RUSTFLAGS =
#RUSTFLAGS += -C linker=lld
#RUSTFLAGS += -Z linker-flavor=ld.lld
#RUSTFLAGS += -C link-arg=-nostartfiles
#RUSTFLAGS += --emit asm,llvm-bc,llvm-ir,obj,metadata,link,dep-info,mir


all: $(HEX_FILES)

clean:
	xargo clean
	rm -f $(HEX_FILES)
	find . \( -name \*~ -o -name \*.rs.bk \) -exec rm {} \;

define d_b_rules =
build_$(1):: build_$(1)_$(2)

build_$(1)_$(2):
	RUSTFLAGS="-C link-arg=-Tlayout_$(1).ld $(RUSTFLAGS)" \
		xargo build $(CARGOFLAGS) --target $($(1)_TARGET) --features "$(1)" --bin $(2)

$(1)_$(2).hex: build_$(1)_$(2)
	arm-none-eabi-objcopy -O ihex target/$($(1)_TARGET)/release/$(2) "$(1)_$(2)".hex
endef
$(foreach D,$(DEVICES),$(foreach B,$(BINARIES),$(eval $(call d_b_rules,$D,$B))))

flash_%: %.hex
	lpc21isp -control "$*".hex /dev/ttyUSB0 115200 12000
