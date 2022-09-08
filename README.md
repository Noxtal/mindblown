# mindblown
A `brainf**k` to `x86 ELF` compiler written in Rust meant for Linux and Windows under WSL. Codegen is made from brainf\*\*k directly to x86 Intel assembly without any IR. Most of the optimization work is done when parsing, so an integrated interpreter can benefit from it too. 

This is in a very early state. Stay tuned for more features and optimizations like the ones described in [TODO](#todo).

> Note:
`samples/patterns.py` was solely used for detecting common loop patterns in classical brainf\*\*k programs for present and future refactoring.

## Table of Contents
- [mindblown](#mindblown)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Getting started](#getting-started)
    - [Debug using the integrated interpreter](#debug-using-the-integrated-interpreter)
  - [Inspiration](#inspiration)
  - [TODO](#todo)

## Prerequisites
* [Rust](https://www.rust-lang.org/tools/install)
* [GCC](https://gcc.gnu.org/install/)
* [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) (for Windows users)


## Getting started
1. Get the latest binary release installed on your machine
```sh
# Get the latest cargo release
$ cargo install mindblown

# Build from source
$ git clone https://github.com/Noxtal/mindblown.git
$ cd mindblown
$ cargo build --release
```

2. Use the `mindblown` command to compile your brainf\*\*k program
```sh
$ mindblown <FILENAME>
# The resulting binary will be named as the input program BUT without extension

$ chmod +x <OUTPUT>
$ ./<OUTPUT>
```

(`<OUTPUT>`, )

> YOU ARE NOW READY TO USE MINDBLOWN! 🎉


### Debug using the integrated interpreter
```bash
$ mindblown
```

> Note: The integrated interpreter uses the [snailquote](https://github.com/euank/snailquote) syntax for input. Please refer to their [documentation](https://docs.rs/snailquote/latest/snailquote/fn.unescape.html) for more information.

## Inspiration
* https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#intro-to-x86 for the x86 assembly codegen
* https://github.com/pretzelhammer/brainfuck_compilers/blob/master/src/x86_64/compiler.rs for examples of the x86_64 assembly codegen in Rust
* http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html for optimizing brainf\*\*k
* http://brainfuck.org for most programs in the `samples` folder


## TODO
- [ ] Add various CLI features such as choosing the output file, etc.
- [ ] Implement clearer and smarter error handling
- [ ] Make the interpreter take in newlines
- [ ] Optimize loop handling to Assembly
- [ ] Allow for more optimization