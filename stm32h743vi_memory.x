/* Memory layout for stm32h743vi
see "Memory mapping" table in stm32h743vi data sheet
*/
MEMORY
{
  /* FLASH and RAM are mandatory memory regions */

  /* Backup SRAM */
  BSRAM : ORIGIN = 0x38800000, LENGTH = 4K

  /* SRAM */
  SRAM4 : ORIGIN = 0x38000000, LENGTH = 64K
  SRAM3 : ORIGIN = 0x30040000, LENGTH = 32K
  SRAM2 : ORIGIN = 0x30020000, LENGTH = 128K
  SRAM1 : ORIGIN = 0x30000000, LENGTH = 128K

  /* AXI-SRAM */
  AXISRAM : ORIGIN = 0x24000000, LENGTH = 512K

  /* Data tightly coupled memory */
  DTCM  : ORIGIN = 0x20000000, LENGTH = 128K

  /* reserve some flash for the preinstalled micropython bootloader? */
  BL_FLASH  : ORIGIN = 0x08000000, LENGTH = 256K
  /* this appears to be the app origin that the bootloader calls */
  FLASH  : ORIGIN = 0x08040000, LENGTH = 1792K
  /* use SRAM1 + SRAM2 for main RAM */
  RAM    : ORIGIN = 0x30000000, LENGTH = 256K

  /* Instruction tightly coupled memory: used for stack */
  ITCM  : ORIGIN = 0x00000000, LENGTH = 64K
}

SECTIONS
{
    .dtcm (NOLOAD) : ALIGN(4)
    {
        *(.dtcm .dtcm.*);
        . = ALIGN(4);
    } > DTCM

    .axisram (NOLOAD) : ALIGN(4)
    {
        *(.axisram .axisram.*);
        . = ALIGN(4);
    } > AXISRAM

} INSERT AFTER .bss;

/* The location of the stack can be overridden using the
   `_stack_start` symbol.  Place the stack at the end of RAM */
_stack_start = ORIGIN(ITCM) + LENGTH(ITCM);

/* The location of the .text section can be overridden using the
   `_stext` symbol.  By default it will place after .vector_table */
/* _stext = ORIGIN(FLASH) + 0x40c; */

