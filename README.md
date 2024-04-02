# Dashlang

A interpreted language focused on simplicity and consistency

## Types

This minimalist language only includes some basic types:

Name | example
-- | --
Integer | 1
Float | 1.0
String | "Foo"
Boolean | true
Null | Null
Closure | `(n) {return n + 1}`

## Syntax

### Declaring a variable

Declaring a variable is as simple as choosing a variable name and assigning a value to it:
```
name = "Gabriel"
```

### Functions

There is no functions in this language, we use instead closures assigned to variables:

```
increment = (n) {return n + 1}
```

We call the function the same way as most of the languages:

```
n = 0
n = increment(n)
```

You can also use the pipelining operator to chain function calls:

```
increment = (n) {
  return n + 1
}

n = 0
n = n
|> increment()
|> increment()
|> increment()

println(n)
```

### Loops and conditionals

Currently, we have `if`, `while` and `for` statements:

```
while count > 0 {
    ...
}
if count > 0 {
    ...
}
for count = 0; count < 10; count += 1 {
    ...
}
```

As you can see in the `for` example, we also have the compound assignment expression operator (`+=`, `-=`, etc...)
