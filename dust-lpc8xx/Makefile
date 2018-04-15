TARGETS =
TARGETS += lpc810 lpc812 lpc824 lpc845

EXECUTABLE = test_port

BUILDS = $(addprefix build-, $(TARGETS))
HEX_FILES = $(addsuffix _$(EXECUTABLE).hex, $(TARGETS))


all: $(HEX_FILES)

clean:
	xargo clean
	rm -f $(HEX_FILES)
	find . \( -name \*~ -o -name \*.rs.bk \) -exec rm {} \;

build-%:
	RUSTFLAGS="-C link-arg=-Tlayout_$*.ld" xargo build --release --target thumbv6m-none-eabi --features "$*"

%_$(EXECUTABLE).hex: build-% 
	arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/release/$(EXECUTABLE) "$*"_$(EXECUTABLE).hex

flash-%:
	lpc21isp -control "$*".hex /dev/ttyUSB0 115200 12000
