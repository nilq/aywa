# kravl
An compiled programming language in Rust.

## Syntax

Functions
```
define add(a b) -> int do
    a + b
end

define not_zero?(a) -> bool do
    add(a, 100) > 100
end
```

Functional
```
define higher_order(f x) -> func do
    lambda a: f(x + a)
end
    
foo = lambda x: println("yo, ", x)
higher_order(foo)(100)
```
