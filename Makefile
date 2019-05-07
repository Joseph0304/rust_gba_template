ifeq ($(strip $(DEVKITARM)),)
$(error "Please set DEVKITARM in your environment. export DEVKITARM=<path to>devkitARM")
endif

include $(DEVKITARM)/gba_rules

export CARGO     := xargo
export LD        := ld.lld

TARGET     := $(notdir $(CURDIR))
TARGET_DIR := $(CURDIR)/target/gba
GBALIB_DIR := $(DEVKITARM)/arm-none-eabi/lib

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
