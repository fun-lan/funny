# Pair

```elm
pair : a . b . (a . b . c) . c
  = a b f . f a b
```

Contrary to what you might be thinking, `pair` is not really a type -- It's a
function. It is quite clever too! It accepts two generic elements (`a` and `b`)
and a function `f` that it applies to `a` and `b`.

There are two functions in particular which are essential to work with a pair.

```elm
fst : a . b . a
  = a b . a

snd : a . b . b
  = a b . b
```

They accept the elements of a pair and return either the first or the second
element from it.

There is, of course `type alias Pair` defined. Here it is:

```elm
type alias Pair a b = (a . b . c) . c
```

With this type alias, you can now define functions and types on Pairs like so:

```elm
type alias Point = Pair Float Float

distance : Point . Point . Float
  = undefined
```
