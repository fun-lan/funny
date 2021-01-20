# Planning

Minimalistic grammar for the first **untyped** implementation.

```python
alias := identifier EQ expr

identifier := id_char | id_char identifier
id_char := UNDERSCORE | LOWER

expr := atom | identifier | operator | lambda | application

atom := str

str := DTICK str_body DTICK
str_body := UTF8 | UTF8 str_body

operator := expr SIGN expr

lambda := binds DOT expr
binds := identifier | identifier binds

application := expr expr

# Constants.
LOWER := 'a'..'z'
UNDERSCORE := '_'
DTICK := '"'
UTF8 := \x00..\xFF\xFF\xFF\xFF  # All available UTF-8 codepoints.

EQ := "="
DOT := "."
SIGN := "++"
```
