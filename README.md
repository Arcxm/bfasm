# bfasm

bfasm is a brainfuck to assembly converter written in Rust.

Please note that the project currently only generates Windows x86-64 assembly in Intel syntax!

## Quick Start

You can try one of the Brainfuck programs supplied in [tests](tests) or one of your own.

```console
$ cargo run filename.bf
```

Which will generate `filename.asm`.

### Assembling

Using `nasm` and the Visual Studio Developer Command Prompt:

```console
$ nasm -fwin64 filename.asm
$ link filename.obj msvcrt.lib /LARGEADDRESSAWARE:NO
$ ./filename
```

## TODOs

- flag to generate 32 bit Assembly?
- flag to generate Assembly in AT&T Syntax
- flag to generate Assembly for Linux
- fix `/LARGEADDRESSAWARE:NO` when linking