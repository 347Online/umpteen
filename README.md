# The Umpteen Programming Language

This repository provides the reference implementation for the Umpteen Programming Language, bootstrapped from Rust.
Umpteen is currently in active development and frequent breaking changes are expected until v1.0.x

As such, Umpteen is not yet recommended for use in production

# Syntax

## Comments

Write a line comment with `#` or a block comment with opening and closing `###`

```umpteen
# This is a line comment

###
This is a block comment
Continued over multiple
lines
###
```

## Variables

Create mutable or immutable\* bindings with `var` and `let` respectively

```umpteen
var x = 10; # x is mutable
let y = 20; # y is immutable
x = 0; # OK ✅
y = 0; # ERROR 🚫
```

_\*NOTE: Immutable bindings are not yet implemented_

### Scope

```umpteen
# Global Scope

let a = 10;
print(a); # 10
{
  # Block-scope
  print(a); # 10

  let a = 20;
  print(a); # 20
}

print(a); # 10
```

### Shadowing\*

Immutable bindings support shadowing within the same scope

```umpteen
let a = 10; # OK ✅
let a = 20: # OK ✅
a = 30; # ERROR 🚫
```

_\*NOTE: Shadowing within the same scope is not yet implemented_

Mutable bindings can be reassigned, however they are not permitted to be shadowed within the same scope. Conversely, shadowing is permitted within a narrower scope

```umpteen
var a = 10; # OK ✅
{
  var a = 20; # OK ✅
}
a = 30; # OK ✅
var a = 40 # ERROR 🚫

```

## Conditionals

Test an expression with the `if` keyword

```umpteen
if true {
  print("This code prints!"); # ✅
}

if false {
  print("This code is unreachable"); # ⛔
}
```

As well as `else` and `else if`

```umpteen
let something = false;
let somethingElse = false;

if something {
  print("Something!");
} else if somethingElse {
  print("Something else!");
} else {
  print("Neither!");
}
```

## Loops

Execute statements multiple times with the `loop` keyword. Use `break` to exit early from the loop body, or `continue` to halt execution of the current iteration and skip to the next one

```umpteen
var i = 0;
loop {
  print(i);
  i += 1;

  if i > 10 {
    break;
  }
}
```

## Functions

Declare a function with the `fnc` keyword. Parameters require type annotations. Annotations for return types are required, unless the function returns `Empty`

```umpteen
fnc fib(n: Number) -> Number {
  if n <= 1 {
    return n - 1;
  }

  return n + fib(n - 1);
}
```

## Data Types\*

- `Empty`: No value
- `Boolean`: `true` or `false`
- `Number`: [IEEE 754](https://en.wikipedia.org/wiki/Double-precision_floating-point_format) double-precision floating point representation of numerics
- `String`: A series of characters
- `Object`: Compound data types passed by reference instead of by value
  - `Fnc`: Function type representing a discrete collection of executable instructions  
    _NOTE: User-defined functions are not yet implemented_
  - `List`: Dynamic Array type, representing a one-dimensional dynamically resizeable numerically indexed collection

_\*NOTE: The full specification for Umpteen's type system is not yet defined, definition of all types is subject to change prior to v1.0.x_

---

**_WARNING: Umpteen is still in active development and frequent breaking changes are expected prior to v1.0.x. Not yet recommended for production use!_**
