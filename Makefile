ifeq ($(strip $(DEVKITARM)),)
$(error "Please set DEVKITARM in your environment. export DEVKITARM=<path to>devkitARM")
endif

include $(DEVKITARM)/gba_rules

export CARGO     := xargo
export LD        := $(CC)

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
	@$(CARGO) build --release
	@$(LD) $(LDFLAGS) -specs=$(GBALIB_DIR)/gba.specs $(TARGET_DIR)/release/lib$(TARGET).a -o $(TARGET).elf
	@$(CARGO) objcopy -- -O binary $(TARGET).elf $(TARGET).gba
	@gbafix $(TARGET).gba

build-debug:
	@$(CARGO) build
	@$(LD) $(LDFLAGS) -specs=$(GBALIB_DIR)/gba.specs $(TARGET_DIR)/release/lib$(TARGET).a -o $(TARGET).elf
	@$(CARGO) objcopy -- -O binary $(TARGET).elf $(TARGET).gba
	@gbafix $(TARGET).gba

clean:
	@echo clean ...
	@$(CARGO) clean
	@rm -fr $(TARGET).elf $(TARGET).gba
