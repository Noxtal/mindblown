# mindblown
Brainfuck to x86 ELF compiler with batteries included written in Rust meant for Linux and Windows under WSL. Code generation is made from brainf\*\*k directly to x86 Intel assembly as IR. Most of the optimization work is done when parsing, so an integrated interpreter can benefit from it too. 

This is in a very early state. Stay tuned for more features and optimizations like the ones described in [TODO](#todo).

> Note:
`samples/patterns.py` was solely used for detecting common loop patterns in classical brainf\*\*k programs for present and future refactoring.

## Table of Contents
- [mindblown](#mindblown)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Getting started](#getting-started)
  - [Usage](#usage)
    - [Compile a brainf\*\*k program](#compile-a-brainfk-program)
  - [Inspiration](#inspiration)
  - [TODO](#todo)

## Prerequisites
* [Rust](https://www.rust-lang.org/tools/install)
* [GCC](https://gcc.gnu.org/install/)
* [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) (for Windows users)


## Getting started
Get the latest binary release installed on your machine
```sh
# Get the latest cargo release
$ cargo install mindblown

# Build from source
$ git clone https://github.com/Noxtal/mindblown.git
$ cd mindblown
$ cargo build --release
```

## Usage
```sh
mindblown 0.3.0
Brainfuck to x86 ELF compiler with batteries included.

USAGE:
    mindblown [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    compile    Compiles a Brainfuck file.
    help       Print this message or the help of the given subcommand(s)
```

> If no subcommand is provided, `mindblown` will default to the intergrated interpreter, which also uses the [snailquote](https://github.com/euank/snailquote) syntax for input. Please refer to their [documentation](https://docs.rs/snailquote/latest/snailquote/fn.unescape.html) for more information.

### Compile a brainf\*\*k program
```sh
mindblown-compile
Compiles a Brainfuck file.

USAGE:
    mindblown compile [OPTIONS] <FILE>

ARGS:
    <FILE>    The Brainfuck file to compile.

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    The output file.
    -r, --run                Run the compiled executable after compiling.
```

## Inspiration
* https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#intro-to-x86 for the x86 assembly codegen
* https://github.com/pretzelhammer/brainfuck_compilers/blob/master/src/x86_64/compiler.rs for examples of the x86_64 assembly codegen in Rust
* http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html for optimizing brainf\*\*k
* http://brainfuck.org for most programs in the `samples` folder


## TODO
- [x] Make the interpreter take in newlines
- [x] Add various CLI features such as choosing the output file, etc.
- [x] Implement clearer and smarter error handling
- [ ] Optimize loop handling to Assembly
- [ ] Allow for more optimization