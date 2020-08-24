/* Memory layout for stm32h743zit6u (on NUCLEO-H743ZI2) */
MEMORY
{
  /* FLASH and RAM are mandatory memory regions */

  /* Backup SRAM */
  /* BSRAM : ORIGIN = 0x38800000, LENGTH = 4K */

  /* SRAM */
  /*
  SRAM4 : ORIGIN = 0x38000000, LENGTH = 64K
  SRAM3 : ORIGIN = 0x30040000, LENGTH = 32K
  SRAM2 : ORIGIN = 0x30020000, LENGTH = 128K
  SRAM1 : ORIGIN = 0x30000000, LENGTH = 128K
  */

  /* AXI-SRAM */
  /* AXISRAM : ORIGIN = 0x24000000, LENGTH = 512K */

  /* Data tightly coupled memory */
  /* DTCM  : ORIGIN = 0x20000000, LENGTH = 128K */

  /* the breakout board does not include the micropython bootloader */
  /* Use the same amount of FLASH1 + FLASH2 */
  FLASH  : ORIGIN = 0x8000000, LENGTH = 1792K

  /* use AXISRAM for main RAM */
  RAM    : ORIGIN = 0x24000000, LENGTH = 512K

  /* Instruction tightly coupled memory: used for stack */
  ITCM  : ORIGIN = 0x00000000, LENGTH = 64K
}

/* The location of the stack can be overridden using the
   `_stack_start` symbol.  We use ITCM for stack */
_stack_start = ORIGIN(ITCM) + LENGTH(ITCM);

/* The location of the .text section can be overridden using the
   `_stext` symbol.  By default it will place after .vector_table */
/* _stext = ORIGIN(FLASH) + 0x40c; */

