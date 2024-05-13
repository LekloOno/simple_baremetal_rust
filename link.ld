MEMORY
{
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 512K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}
__reset_stack_pointer = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .text : {
        LONG(__reset_stack_pointer);
        LONG(_start | 1);
        /* The whole interrupt table is 332 bytes long. Advance to that position. */
        . = 500;
        /* And here comes the rest of the code */
        *(.text*)
    } > FLASH /* Put this in the flash memory region */
}