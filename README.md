# Introduction

> ⚠️
> **The language is not ready yet!**
> ⚠️

The [documentation](doc) is, _de facto_ a design draft of what's coming. I will
be building this _incrementally_, starting with an **untyped** basic version of
the language with a minimalistic standard library and **essential** syntax.

You will be able to install `funny` with `cargo install` as described below,
and even run it, but it's pretty useless at this point. I am actively working
on this project and will be posting updates though, so stay tuned!

## Progress

Version: `0.1.0-alpha` (upcoming).

- [ ] Compiler
    - [ ] Grammar
    - [ ] CharStream
    - [ ] Lexer
    - [ ] Parser
    - [ ] Abstract Syntax Tree
- [ ] Runtime
    - [ ] Reducer
    - [ ] Runner
    - [ ] Base (Standard Library)
    - [ ] `funny run`

## Install

For now, the only way to install Funny is through [cargo][1] (Rust package
manager). Yet, it's a sexy way to do it:

```bash
cargo install funny
```

[1]: https://doc.rust-lang.org/cargo/getting-started/installation.html

## Hello, World!

```hs
-- main.fu
main : [String] . String
  = _ . "Hello, World!"
```

Run this using:

```bash
funny run main.fu
```
