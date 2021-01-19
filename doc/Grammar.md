# Grammar

Funny files consist of a series of statements:

- Function alias
- Type alias `type alias`
- Type definition `type`
- Trait definition `trait`
- Trait implementation `impl`

As you can see, there isn't much to it. You may also notice, that there is no
_function definition_ statement, only _function alias_. That is because
functions are defined by lambdas and lambdas **only**.

Let's look at each statement separately!

## Function Alias

In Funny, all functions are actually just lambdas. Some of them have names. To
name a function, you simply provide an alias.

```elm
add = a b . a + b
```

This is an example of an untyped alias. Under the hood, it still has a type --
compiler will infer as much as possible from context, however we can explicitly
specify the type through a type signature.

```elm
add : a . a . a | a : Num
  = a b . a + b
```

Here, we can see that the `add` function takes two generic `a` types and return
their sum. The only constraint on the `a` type is that it has to be `Num`eric.

## Type Alias

Sometimes, we have a type that represents something specific. For example, we
might have a function like this:

```elm
distance : (Float, Float) . (Float, Float) . Float
  = undefined -- implementation is beyond the point
```

It takes two points in Euclidian space characterised by their coordinates and
returns their distance. In this case, the pair of `(Float, Float)` is,
conceptually, its own type -- `Point`. So let's reflect that in our code!

```elm
type alias Point = (Float, Float)
```

Now, we can be more expressive with our type signature.

```elm
distance : Point . Point . Float
  = undefined -- implementation is beyond the point
```

## Traits

Traits are kind of like interfaces in other programming languages. In functional
programming, these are usually refered to as _type classes_ (Haskell) or traits
(Rust).

Traits help us reason about different types in our system. For example, there
are some types that can be printed out. Such types must implement the `Show`
trait demonstrated below.

### Trait Definition

```elm
trait Show a where
  show : a . String
```

If you wish to print your type using the `io.println!` function from the
[standard library](Core/README.md), you must implement the `Show` trait for it.

### Trait Implementation

```elm
impl Show for Maybe a | Show a where
  show : Maybe a . String
    = maybe
    . case maybe of
      None   -> "None"
      Some a -> "Some " ++ show a
```

Just like function aliases, trait implementations can have generic limits to
them. It is apparent that we cannot `show` a `Some x` if that `x` does not
implement `Show`. How would we know how to display it? So we must specify that
the `Show` trait is only applicable to such instances of `Maybe` where the
contained value is itself showable.
