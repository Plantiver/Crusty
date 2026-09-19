OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

/* Tell the linker we want our kernel loaded in the higher half of memory */
PHDRS
{
    headers PT_PHDR PHDRS;
    requests PT_LOAD;
    text PT_LOAD;
    rodata PT_LOAD;
    data PT_LOAD;
}

SECTIONS
{
    /* Limine will load the kernel at this physical address, mapped to high virtual memory */
    . = 0xffffffff80000000;

    /* This is the critical part: It guarantees your Limine requests are placed EXACTLY here */
    /* Update this block to include the start and end markers */
    .requests : {
        KEEP(*(.requests_start_marker))
        KEEP(*(.requests))
        KEEP(*(.requests_end_marker))
    } :requests

    . = ALIGN(0x1000);
    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(0x1000);
    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(0x1000);
    .data : {
        *(.data .data.*)
    } :data

    . = ALIGN(0x1000);
    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    } :data
    
    /* Discard standard unused sections */
    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}
