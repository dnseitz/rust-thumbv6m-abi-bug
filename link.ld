MEMORY
{
  RAM (xrw)     : ORIGIN = 0x20000000, LENGTH = 6K
  FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 32K
}

stack_size = 1024;

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

ENTRY(start);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(_stack_start)
    KEEP(*(.reset))
    KEEP(*(.exceptions))
  } > FLASH

  .text :
  {
    . = ALIGN(4);
    *(.text)
    *(.text*)
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx*)
    *(.note.gnu.build-id*)
  }
}

