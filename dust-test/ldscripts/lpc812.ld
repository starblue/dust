/* NXP LPC812 */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 16K
  RAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 4K
}

INCLUDE sections.ld
