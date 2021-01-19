# List

```elm
type List a
  = Null
  | Cons a (List a)
```

Here it is, quite simple and elegant. `List` is a recursive type, as you can
see. It also has a few associated functions.

```elm
null : List a . Bool
  = list
  . case list of
    Null     -> True
    Cons _ _ -> False
```

See definition of `Bool` [here](Bool.md).

```elm
(:) : a . List a . List a
  = a list . Cons a list

(++) : List a . List a . List a
  = l1 l2
  . case l1 of
    Null      -> l2
    Cons x xs -> x : xs ++ l2

head : List a . Maybe a
  = list
  . case list of
    Null     -> None
    Cons x _ -> Some x

tail : List a . List a
  = list
  . case list of
    Null      -> Null
    Cons _ xs -> xs
```

See definition of `Maybe` [here](Maybe.md).
