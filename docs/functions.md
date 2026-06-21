# Functions

## Table of Contents

- [Function Declaration](#function-declaration)
- [Parameters](#parameters)
- [Return Values](#return-values)
- [Arrow Functions (Lambdas)](#arrow-functions-lambdas)
- [Closures](#closures)
- [Higher-Order Functions](#higher-order-functions)
- [Recursive Functions](#recursive-functions)
- [Async Functions](#async-functions)
- [Default Parameters](#default-parameters)
- [Function Composition](#function-composition)

## Function Declaration

Use `func` to declare a named function:

```nexora
func greet(name) {
    print "Hello, " + name + "!"
}

greet("Nexora")  // Output: Hello, Nexora!
```

### Functions with Return Values

Use `return` to return a value:

```nexora
func add(a, b) {
    return a + b
}

let result = add(5, 3)  // 8
print result
```

### No-Argument Functions

```nexora
func sayHello() {
    print "Hello!"
}

sayHello()
```

## Parameters

Functions accept zero or more parameters:

```nexora
func describe(name, age, city) {
    print name + " is " + str(age) + " years old and lives in " + city
}

describe("Ashish", 20, "Mumbai")
```

## Default Parameters

Parameters can have default values:

```nexora
func greetFormal(name, title = "Mr.") {
    print "Hello, " + title + " " + name
}

greetFormal("Smith")          // Hello, Mr. Smith
greetFormal("Johnson", "Dr.") // Hello, Dr. Johnson
```

## Return Values

The `return` statement exits the function and returns a value:

```nexora
func max(a, b) {
    if a > b {
        return a
    }
    return b
}

let result = max(10, 20)  // 20
```

If no `return` is specified, the function returns `null`.

## Arrow Functions (Lambdas)

Arrow functions provide a concise syntax for anonymous functions:

### Single Parameter

```nexora
let double = x => x * 2
print double(5)  // 10
```

### Multiple Parameters

```nexora
let add = (a, b) => a + b
print add(3, 4)  // 7
```

### Multi-Statement Body

```nexora
let calculate = (a, b) => {
    let sum = a + b
    let product = a * b
    return sum + product
}
```

### As Function Arguments

```nexora
let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, x => x * 2)
print doubled  // [2, 4, 6, 8, 10]
```

## Closures

Closures capture variables from their enclosing scope:

```nexora
let multiplier = 3
let multiply = x => x * multiplier

print multiply(5)   // 15
print multiply(10)  // 30
```

### Closure State

Closures maintain their own state:

```nexora
let counter = 0
let increment = func() {
    counter = counter + 1
    return counter
}

print increment()  // 1
print increment()  // 2
print increment()  // 3
```

### Factory Functions

```nexora
func makeAdder(n) {
    return x => x + n
}

let add5 = makeAdder(5)
let add10 = makeAdder(10)

print add5(3)   // 8
print add10(3)  // 13
```

## Higher-Order Functions

Functions can accept and return other functions:

```nexora
func apply(func, value) {
    return func(value)
}

let double = x => x * 2
let result = apply(double, 5)  // 10
print result
```

### Map

Transform each element in an array:

```nexora
let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, x => x * 2)
print doubled  // [2, 4, 6, 8, 10]
```

### Filter

Select elements that match a condition:

```nexora
let numbers = [1, 2, 3, 4, 5, 6]
let evens = filter(numbers, x => x % 2 == 0)
print evens  // [2, 4, 6]
```

### Reduce

Accumulate a single value:

```nexora
let numbers = [1, 2, 3, 4, 5]
let sum = reduce(numbers, (acc, x) => acc + x, 0)
print sum  // 15
```

## Recursive Functions

Functions can call themselves:

```nexora
func factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print factorial(5)   // 120
print factorial(10)  // 3628800
```

### Fibonacci Sequence

```nexora
func fibonacci(n) {
    if n <= 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

for i in range(0, 10) {
    print "fib(" + str(i) + ") = " + str(fibonacci(i))
}
```

## Async Functions

Use `async` to declare an asynchronous function:

```nexora
async func fetchData(url) {
    print "Fetching data from " + url + "..."
    let response = await http.get(url)
    return response.json()
}
```

Use `await` to wait for an async operation:

```nexora
let data = await fetchData("https://api.example.com/data")
```

See [Error Handling](error-handling.md) for handling async errors with try-catch.

## Function Composition

Combine multiple functions into one:

```nexora
func compose(f, g) {
    return x => f(g(x))
}

let addTen = x => x + 10
let timesTwo = x => x * 2

let addTenThenDouble = compose(timesTwo, addTen)
print addTenThenDouble(5)  // 30 (5 + 10 = 15, 15 * 2 = 30)
```

## Examples

### Calculator

```nexora
func calculate(a, b, op) {
    if op == "+" { return a + b }
    if op == "-" { return a - b }
    if op == "*" { return a * b }
    if op == "/" { return a / b }
    return null
}

print calculate(10, 5, "+")  // 15
print calculate(10, 5, "*")  // 50
```

### Function as Data

```nexora
let operations = {
    add: (a, b) => a + b,
    sub: (a, b) => a - b,
    mul: (a, b) => a * b
}

print operations.add(2, 3)  // 5
print operations.mul(4, 5)  // 20
```
