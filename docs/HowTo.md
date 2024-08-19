# How To

1. Parse the linker.map file, which is a text file that contains the memory map of the program.

2. The memory map has sections that are defined by the linker script. The sections are defined by the `SECTIONS` command in the linker script. The Symbol Table is in the "Symbols" section.

3. The Symbol Table contains the following columns:
    - Address
    - Size
    - File
    - Name

4. The symbols are mangled by default. So, we need to an optional way to demangle them specifically for Rust symbols.


## Open Questions

1. Should we use a custom parser or a library to parse the linker map file?
2. Is there a standard way to demangle Rust symbols?
3. How should we display the memory map? Should we use a tree view or a table view? Do we want it to be interactive? How fancy do we want it to be?
4. Should we support other formats like ELF or DWARF?
5. Visualizing the memory map is the main goal. But, should we also provide other features like searching for symbols, filtering, etc.?
