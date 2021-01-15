# Introduction

> Here's a quick introduction of Funny.

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
