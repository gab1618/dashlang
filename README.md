![dashlang](https://github.com/GabrielBrandao1618/dashlang/assets/62272513/378f8105-2016-44e1-a573-0e7da9699680)

# Dashlang

A minimalist programming language focused on simplicity and consistency

## Documentation

### Types

This minimalist language only includes some basic types:

Name | example
-- | --
Integer | 1
Float | 1.0
String | "Foo"
Boolean | true
Null | null
Closure | `(n) {return n + 1}`
Vector | `[1, 2, 3]`
Tuple | `("ok", 3)`
Map | `{name: "John", age: 25}`

### Syntax

#### Declaring a variable

Declaring a variable is as simple as choosing a variable name and assigning a value to it:
```
name = "Gabriel"
```

#### Functions

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

#### Loops and conditionals

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


## Features

- Basic types, such as integers, floats, booleans and others
- Standard library with some basic implementations
- Fancy error diagnostics


## Running Tests

To run the tests, use the following command

```bash
  cargo test --workspace
```

That will run all the tests from every package


## Run Locally

### Requirements
* Rust >= 1.76.x

After cloning the repository, you can run the cli package with the following command

```bash
  cargo run --package cli "<path-to-file>"
```

Where `path-to-file` is the path to the file you want to run. In the examples folder there are already a few examples you can try.

cli is the only package you can actually run, all the other packages are just libraries.
## Installation

You can download the cli tool on the releases tab

https://github.com/GabrielBrandao1618/dashlang/releases
