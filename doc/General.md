# General

> This folder contains different documentation files (and sub-folders) for the
> Funny programming language.

## Overview

**Funny** is a functional programming language for server-side software
development. Its main objectives are:

1. Developer friendly and easy to learn.
   - Helpful compiler with good suggestions;
   - Expressive syntax;
   - Rich with concepts but simple to understand.
2. Efficient and thread-safe.
   - Immutable and persistent data structures;
   - Comprehensive error handling that eliminates most runtime exceptions;
   - Clear distinctions between pure and non-pure functions.
3. Simple execution pipeline.
   - Ships as a single executable;
   - Can be easily installed with `cargo` from [crates.io](https://crates.io/);
   - Production ready with all tools included.

## Packages

Most of your Funny code will lie within different packages. Most languages use
single files as namespaces. However, that may make your code cluttered, dense,
and too long.

Funny uses `folders` as package containers. Stick a `.fp` extension to the end
of a folder's name to let Funny know that it is a Funny Package (FP). For
example, you could create a `db.fp` folder in your project; all Funny source
code files `.fun` are now part of a single package called `db`.

I think it's convenient. You don't have to specify

```elm
module DB (addUser, delUser, user'sAge) where
```

in every source code file.

## Executables

If you think about it, program's entrypoint is not too different from any other
function. Most languages insist that it has to be called `main`, but that's just
a convention. The only real constraints on an entrypoint function are

1. Parameters: none _or_ a list of strings;
2. Possible return type:
   - Empty return (e.g. `void` or `()`);
   - An **integer** status code for the shell;
   - Something **printable**?

Yeah, why not!? If `ghci` or Python REPL can evaluate a function and print its
result, why can't we just write

```bash
funny run main.fun --entry cli
```

to make Funny use function `cli` as an entrypoint. Funny will default to `main`
by convention, but don't forget about this trick!

In general, all entrypoint functions must adhere to the following type signature

```elm
foo : [String] . p | p : Show
```

The above reads as

> Function `foo` expects one parameter that is a list of strings `[String]` and
> returns some polymorphic type `p` that implements the `Show` trait (in other
> words, we can print it).

```elm
main : [String] . ()    -- type signature for main
  = names               -- bind [String] to names
  . "Hello, "           -- return "Hello, " with appened ...
  ++ case names of      -- (pattern match on names)
    []      -> "World"  -- ... "World" if names is empty or
    [name,] -> name     -- "<name>" appended
  |> io.println         -- pass to io.println for printing
```

To understand what's going on, you need to understand Funny _lambda expressions_
first. Check this out: [Lambdas](Lambda.md).
