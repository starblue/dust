TARGETS =
TARGETS += lpc810 lpc812 lpc824 lpc845
#TARGETS += lpc1114

BUILDS = $(addprefix build-, $(TARGETS))
HEX_FILES = $(addsuffix .hex, $(TARGETS))


all: $(HEX_FILES)

clean:
	xargo clean
	rm -f $(HEX_FILES)
	find . \( -name \*~ -o -name \*.rs.bk \) -exec rm {} \;

build-%:
	xargo build --release --target thumbv6m-none-eabi --features "$*"

%.hex: build-%
	arm-none-eabi-objcopy -O ihex target/thumbv6m-none-eabi/release/dust-test-uart "$*".hex

flash-%:
	lpc21isp -control "$*".hex /dev/ttyUSB0 115200 12000
