# kravl
An interpreted programming language in Rust.

## Syntax

Functions
```
define add(a b) -> int
    a + b

define not_zero?(a) -> bool
    add(a, 100) > 100
```

Functional
```
define higher_order(f x) -> func
    lambda a: f(x + a)

foo = lambda x: println("yo, ", x)
higher_order(foo)(100)
```
