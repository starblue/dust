/* NXP LPC845 */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 64K
  RAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 16K
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
}
