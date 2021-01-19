# Maybe

## Definition

```elm
type Maybe a
  = None
  | Some a
```

## Implementations

### Show

```elm
impl Show for Maybe a | Show a where
  show : Maybe a . String
    = maybe
    . case maybe of
      None   -> "None"
      Some a -> "Some " ++ show a
```
