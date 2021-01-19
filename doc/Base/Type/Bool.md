# Bool

```elm
type Bool
  = False
  | True
```

I don't thing I need to explain myself here. The `if` function is defined as
follows:

```elm
if : Bool . a . a . a
  = b x y
  . case b of
    False -> y
    True  -> x
```

Notice that the two elements/expressions given to `if` must be of the same
type. It is enforced by the compiler due to the type signature of `if`.
