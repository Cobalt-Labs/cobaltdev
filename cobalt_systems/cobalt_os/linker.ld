SECTIONS {
    .text : {
        KEEP(*(.text*))
    }

    .rodata : {
        KEEP(*(.rodata*))
    }

    .data : {
        KEEP(*(.data*))
    }

    .bss : {
        *(COMMON)
        *(.bss*)
    }
}
ENTRY(_start)
