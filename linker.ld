OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

MEMORY {
  rom   : ORIGIN = 0x08000000, LENGTH = 32M
  iwram : ORIGIN = 0x03000000, LENGTH = 32K
  ewram : ORIGIN = 0x02000000, LENGTH = 256K
}

__text_start    = ORIGIN(rom);
__eheap_end     = ORIGIN(ewram) + LENGTH(ewram);
__iwram_start   = ORIGIN(iwram);
__iwram_top     = ORIGIN(iwram) + LENGTH(iwram);;
__sp_irq        = __iwram_top - 0x060;
__sp_usr        = __sp_irq - 0x0a0;
__irq_flags     = 0x03007ff8;

SECTIONS {
  . = __text_start;
  .text  : {
    *(.text)
  } >rom = 0xff
  __text_end = .;

  .data : { *(.data) }
  .bss : { *(.bss) }

  __exidx_start = .;
  .ARM.exidx   : { *(.ARM.exidx*) } >rom
  __exidx_end = .;
}
