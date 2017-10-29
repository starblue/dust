/* NXP LPC812 */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 16K
  RAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 4K
}

SECTIONS
{
  /* Set stack top to end of RAM */
  __StackTop = ORIGIN(RAM) + LENGTH(RAM);

  .text :
  {
    LONG(__StackTop);
    KEEP(*(.cm0plus_vectors));
    KEEP(*(.irq_vectors));

    *(.text*)
  } > FLASH

  .rodata :
  {
    *(.rodata*)
  } > FLASH

  . = ALIGN(4);
  __data_load__ = .;

  .data : AT (__data_load__)
  {
    . = ALIGN(4);
    __data_start__ = .;
    *(.data*)
    . = ALIGN(4);
    __data_end__ = .;
  } > RAM

  .bss :
  {
    . = ALIGN(4);
    __bss_start__ = .;
    *(.bss*)
    . = ALIGN(4);
    __bss_end__ = .;
  } > RAM
}
