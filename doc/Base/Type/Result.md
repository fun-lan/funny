# Result

```elm
type Result err ok
  = Err err
  | Ok ok
```

The `Result` type represents output of some computation where we care to pass
some error context in case of failure. It is sort of like [`Maybe`](Maybe.md),
but more informative.

Let's see an example of where it might be useful.

```elm
factorial : Int . Result Str Int
  = i
  . if (i > 50)
      @ Err "Too big; can't compute"

    @ if (i < 1)
      @ Err "Less than 1; can't compute"

    @ Ok @ product @ range 1 1 i
```
