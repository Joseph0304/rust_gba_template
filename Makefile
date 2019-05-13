export CARGO     := xargo
export LD        := ld.lld

TARGET     := $(notdir $(CURDIR))

.PHONY: usage build build-debug clean

#---------------------------------------------------------------------------------
# options for code generation
#---------------------------------------------------------------------------------
ARCH    :=      -mthumb -mthumb-interwork
LDFLAGS =       -g $(ARCH)

usage:
	@echo "make build"
	@echo "make build-debug"
	@echo "make clean"

build:
	@$(CARGO) objcopy --bin $(TARGET) --release -- -O binary $(TARGET).gba

build-debug:
	@$(CARGO) objcopy --bin $(TARGET) -- -O binary $(TARGET).gba

clean:
	@echo clean ...
	@$(CARGO) clean
	@rm -fr $(TARGET).gba
