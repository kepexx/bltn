# bltn
Basic Language To NWCTB

Usage: `bltn <in> <out> [add lines|false] [file identifier|<in>]`

The "add lines" option adds comments to the resulting output showing the lines of bltn corresponding to the output, for debugging. The file identifier is used for the initial #ifndef check (like C headers)

# What?
`bltn` is used to generate files which can be processed using `nasm -fbin` to create BBJ files for use with `bino` or other interpreters.

The official extension for bltn files is `.bl` or `.bltn` (former preferred). As for output files, it is recommended to leave them with no extension so you are able to use syntax such as `%include "file"` instead of `%include "file.nasm"` which may be confusing as we are coding in bltn not nasm.

# Syntax
Here is an example program demonstrating the syntax:

```
; directly copied to output (comment)
%include "a" ; directly copied to output (NASM preprocessor directive)
!db 0 ; directly copied to output, excluding the ! (any NASM directive or instruction)
~db
~ 0 ; directly copied without a newline
~
; lone ~ makes a newline ^
; be careful to not include comments inbetween ~, as they are directly copied. If we included a comment before the lone ~, we
; would get something like ~db ;comment\n\n

; BBJ:

1 2 3 ; *2 = *1; jmp 3

label1:1 label2:2 label3:3
```
