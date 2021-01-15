# Lambda Expressions

**Lambda expressions** are anonymous functions. Funny is a _functional_
programming language and, henceforth, we rely on lambdas **a lot**! In fact,
every function in Funny is just a _named lambda expression_.

## <a name="main-example"></a> Example

```hs
main : [String] . String  -- function declaration
  = _ . "Hello, World!"   -- function alias
```

Let's take a closer look at this part:

```hs
_ . "Hello, World!"  -- lambda expression
```

As the comment says, this is a basic example of a lambda expression. In other
words, a function. It takes one argument that we immediately ignore by binding
it to the `_underscore`. The dot `.` separates parameters from the return value.
This particular function returns a string literal `"Hello, World!"`.

Coming back to the [main-example](#main-example), we can now say that `main` is
a name alias for a function that takes some argument and returns a string. Well,
that matches `main`'s type signature perfectly! Compiler says "Way to go, man!"

## Partial Application

```hs
concatenate : [String] . String
  = List.reduce (++) ""
```

This function, just like `main`, takes a list of strings and returns a single
string. However, what it does is different -- it concatenates all strings such
that

```hs
>>> concatenate ["Some", "strings", "for", "example"]
"Somestringsforexample"
```

However, you might notice that we didn't use a lambda expression to define this
function... So what happened? How did we do it? Well, let's look at the
`List.reduce` function from the `List` package.

```hs
reduce : (b . a . b) . b . [a] . b
  = op def list .
  case list of
    [x,...xs] = reduce op (op def x) xs
    []        = def
```

As you can see, `reduce` is defined using a lambda expression. Nothing new here.
Some clever [Pattern-Matching](Pattern-Matching.md) and
[Recursion](Recursion.md) is used, but it is still just a lambda.

`reduce` takes a binary operation (a function with two arguments), a default
value, and a list of things. All types are polymorphic, but we can clearly see
that the function takes some `b` and some `a` and returns a `b`.
