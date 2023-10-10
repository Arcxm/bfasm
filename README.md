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

- optimize generated assembly (e.g. multiple `inc`s to one `add` etc.)
- flag to generate 32 bit Assembly?
- flag to generate Assembly in AT&T Syntax
- flag to generate Assembly for Linux
- fix `/LARGEADDRESSAWARE:NO` when linking

## FAQ

### Q: Why does `bfasm` generate X and not Y?

I've only been getting into assembly for the past days, therefore the generated assembly won't be the most optimized or "sane". But that is just an obstacle that can be tackled in the future.

### Q: How does `bfasm` read and write characters?

The generated assembly calls to `_getch` and `putchar` from `libc` as there are no proper syscalls for Windows like on Linux.

### Q: Are there any other Brainfuck-related projects you have been working on?

I've made a [Brainfuck Debugger](https://github.com/arcxm/bfdb) that could be used to debug Brainfuck programs before converting them to assembly.